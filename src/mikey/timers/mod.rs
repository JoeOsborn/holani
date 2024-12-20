pub mod audio_channel_timer;
pub mod base_timer;

use crate::consts::CRYSTAL_TICK_LENGTH;
use crate::mikey::*;
use audio_channel_timer::AudioChannelTimer;
use base_timer::BaseTimer;
use core::num::NonZeroU8;
use log::trace;

const TIMER_TICKS_COUNT: u16 = (0.000001 / CRYSTAL_TICK_LENGTH) as u16; // 1us/62.5ns

const TIMER_LINKS: [Option<NonZeroU8>; 12] = [
    NonZeroU8::new(2),
    NonZeroU8::new(3),
    NonZeroU8::new(4),
    NonZeroU8::new(5),
    None,
    NonZeroU8::new(7),
    None,
    NonZeroU8::new(8),
    NonZeroU8::new(9),
    NonZeroU8::new(10),
    NonZeroU8::new(11),
    NonZeroU8::new(1),
];
const TIMER_COUNT: u8 = 12;

const CTRLA_INTERRUPT_BIT: u8 = 0b10000000;
const CTRLA_RESET_DONE_BIT: u8 = 0b01000000;
#[allow(dead_code)]
const CTRLA_MAGMODE_BIT: u8 = 0b00100000;
const CTRLA_ENABLE_RELOAD_BIT: u8 = 0b00010000;
const CTRLA_ENABLE_COUNT_BIT: u8 = 0b00001000;
const CTRLA_PERIOD_BIT: u8 = 0b00000111;
const CTRLB_TIMER_DONE_BIT: u8 = 0b00001000;
#[allow(dead_code)]
const CTRLB_LAST_CLOCK_BIT: u8 = 0b00000100;
const CTRLB_BORROW_IN_BIT: u8 = 0b00000010;
const CTRLB_BORROW_OUT_BIT: u8 = 0b00000001;

#[derive(Clone, Serialize, Deserialize)]
enum TimerType {
    Base(BaseTimer),
    Audio(AudioChannelTimer),
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TimerReg {
    Backup = 0,
    ControlA,
    Count,
    ControlB,
    Volume,
    Feedback,
    Output,
    ShiftRegister,
}

#[derive(Serialize, Deserialize)]
pub struct Timers {
    timers: [TimerType; TIMER_COUNT as usize],
    timers_triggered: [bool; 8],
    audio_settings: [audio_channel_timer::AudioSettings; 4],
    timer_triggers: [u64; TIMER_COUNT as usize],
    ticks: u64,
}

impl Timers {
    pub fn new() -> Self {
        Self {
            timers: [
                TimerType::Base(BaseTimer::new(0, TIMER_LINKS[0])),
                TimerType::Base(BaseTimer::new(1, TIMER_LINKS[1])),
                TimerType::Base(BaseTimer::new(2, TIMER_LINKS[2])),
                TimerType::Base(BaseTimer::new(3, TIMER_LINKS[3])),
                TimerType::Base(BaseTimer::new(4, TIMER_LINKS[4])),
                TimerType::Base(BaseTimer::new(5, TIMER_LINKS[5])),
                TimerType::Base(BaseTimer::new(6, TIMER_LINKS[6])),
                TimerType::Base(BaseTimer::new(7, TIMER_LINKS[7])),
                TimerType::Audio(AudioChannelTimer::new(8, TIMER_LINKS[8])),
                TimerType::Audio(AudioChannelTimer::new(9, TIMER_LINKS[9])),
                TimerType::Audio(AudioChannelTimer::new(10, TIMER_LINKS[10])),
                TimerType::Audio(AudioChannelTimer::new(11, TIMER_LINKS[11])),
            ],
            timer_triggers: [0; 12],
            audio_settings: [audio_channel_timer::AudioSettings::default(); 4],
            timers_triggered: [false; 8],
            ticks: 0,
        }
    }

    #[inline(always)]
    fn tick_linked_timer(
        timers: &mut [TimerType],
        timers_triggered: &mut [bool],
        audio_settings: &mut [audio_channel_timer::AudioSettings],
        timer_id: NonZeroU8,
    ) -> u8 {
        let timer_id = timer_id.get() as usize;
        match &mut timers[timer_id] {
            TimerType::Base(t) => {
                let (triggered, i) = t.tick_linked(&mut timers_triggered[timer_id]);
                if !triggered {
                    0
                } else {
                    t.linked_timer().map_or(i, |id| {
                        i | Self::tick_linked_timer(timers, timers_triggered, audio_settings, id)
                    })
                }
            }
            TimerType::Audio(t) => {
                let (triggered, i) = t.tick_linked(&mut audio_settings[timer_id - 8]);
                if !triggered {
                    0
                } else {
                    t.linked_timer().map_or(i, |id| {
                        i | Self::tick_linked_timer(timers, timers_triggered, audio_settings, id)
                    })
                }
            }
        }
    }

    pub fn vsync(&mut self) -> bool {
        if self.timers_triggered[2] {
            self.timers_triggered[2] = false;
            return true;
        }
        false
    }

    pub fn hsync(&mut self) -> bool {
        if self.timers_triggered[0] {
            self.timers_triggered[0] = false;
            return true;
        }
        false
    }

    pub fn tick_all(&mut self, current_tick: u64) -> (u8, bool) {
        // bool: Timer 4 has a special treatment, triggered information without interrupt
        let mut int = 0;
        let mut int4_triggered: bool = false;

        self.ticks = current_tick;

        for id in 0..8 {
            if self.timer_triggers[id] > self.ticks {
                continue;
            }
            int |= match &mut self.timers[id] {
                TimerType::Base(t) => {
                    let (triggered, i) = t.tick(self.ticks, &mut self.timers_triggered[id]);
                    if !triggered {
                        0
                    } else {
                        t.linked_timer().map_or(i, |id| {
                            i | Timers::tick_linked_timer(
                                &mut self.timers,
                                &mut self.timers_triggered,
                                &mut self.audio_settings,
                                id,
                            )
                        })
                    }
                }
                TimerType::Audio(_) => {
                    unreachable!();
                }
            };
            self.update_timer_trigger_tick(id);
            if id == 4 {
                int4_triggered = true;
            }
        }
        for id in 8..12 {
            if self.timer_triggers[id] > self.ticks {
                continue;
            }
            int |= match &mut self.timers[id] {
                TimerType::Base(_) => {
                    unreachable!();
                }
                TimerType::Audio(t) => {
                    let (triggered, i) = t.tick(self.ticks, &mut self.audio_settings[id - 8]);
                    if !triggered {
                        0
                    } else {
                        t.linked_timer().map_or(i, |id| {
                            i | Timers::tick_linked_timer(
                                &mut self.timers,
                                &mut self.timers_triggered,
                                &mut self.audio_settings,
                                id,
                            )
                        })
                    }
                }
            };
            self.update_timer_trigger_tick(id);
        }

        (int, int4_triggered)
    }

    fn get_timer(&self, addr: u16) -> (usize, TimerReg) {
        if addr < AUD0VOL {
            (
                ((addr - MIK_ADDR) / 4) as usize,
                match addr % 4 {
                    0 => TimerReg::Backup,
                    1 => TimerReg::ControlA,
                    2 => TimerReg::Count,
                    3 => TimerReg::ControlB,
                    _ => unreachable!(),
                },
            )
        } else {
            (
                (((addr - AUD0VOL) / 8) + 8) as usize,
                match addr % 8 {
                    0 => TimerReg::Volume,
                    1 => TimerReg::Feedback,
                    2 => TimerReg::Output,
                    3 => TimerReg::ShiftRegister,
                    4 => TimerReg::Backup,
                    5 => TimerReg::ControlA,
                    6 => TimerReg::Count,
                    7 => TimerReg::ControlB,
                    _ => unreachable!(),
                },
            )
        }
    }

    #[inline(always)]
    pub fn timer4_interrupt_enabled(&self) -> bool {
        self.peek(TIM4CTLA) & CTRLA_INTERRUPT_BIT != 0
    }

    pub fn peek(&self, addr: u16) -> u8 {
        let (index, cmd) = self.get_timer(addr);
        match &self.timers[index] {
            TimerType::Base(t) => match cmd {
                TimerReg::Backup => t.backup(),
                TimerReg::ControlA => t.control_a(),
                TimerReg::Count => t.count(),
                TimerReg::ControlB => t.control_b(),
                _ => unreachable!(),
            },
            TimerType::Audio(t) => {
                let settings = &self.audio_settings[index - 8];
                match cmd {
                    TimerReg::Backup => t.backup(),
                    TimerReg::ControlA => t.control_a(),
                    TimerReg::Count => t.count(),
                    TimerReg::ControlB => t.control_b(),
                    TimerReg::Volume => settings.volume,
                    TimerReg::Feedback => settings.feedback,
                    TimerReg::Output => settings.output as u8,
                    TimerReg::ShiftRegister => settings.shift_register,
                }
            }
        }
    }

    pub fn poke(&mut self, addr: u16, v: u8) {
        trace!("poke 0x{:04x} -> 0x{:02x}", addr, v);
        let (index, cmd) = self.get_timer(addr);
        match &mut self.timers[index] {
            TimerType::Base(t) => match cmd {
                TimerReg::Backup => t.set_backup(v),
                TimerReg::ControlA => {
                    t.set_control_a(v, self.ticks);
                    self.update_timer_trigger_tick(index);
                }
                TimerReg::Count => {
                    t.set_count(v, self.ticks);
                    self.update_timer_trigger_tick(index);
                }
                TimerReg::ControlB => t.set_control_b(v),
                _ => unreachable!(),
            },
            TimerType::Audio(t) => {
                let settings = &mut self.audio_settings[index];
                match cmd {
                    TimerReg::Backup => {
                        t.set_backup(v);
                        settings.disabled = t.backup() == 0 && settings.feedback == 1;
                    }
                    TimerReg::ControlA => {
                        t.set_control_a(v, self.ticks);
                        self.update_timer_trigger_tick(index);
                    }
                    TimerReg::Count => {
                        t.set_count(v, self.ticks);
                        self.update_timer_trigger_tick(index);
                    }
                    TimerReg::ControlB => t.set_control_b(v),
                    TimerReg::Volume => settings.volume = v,
                    TimerReg::Feedback => {
                        settings.feedback = v;
                        settings.disabled = t.backup() == 0 && settings.feedback == 1;
                    }
                    TimerReg::Output => settings.output = v as i8,
                    TimerReg::ShiftRegister => settings.shift_register = v,
                }
            }
        }
    }

    #[inline(always)]
    pub fn timer_trigger(&self, id: usize) -> u64 {
        match &self.timers[id] {
            TimerType::Base(t) => t.next_trigger_tick(),
            TimerType::Audio(t) => t.next_trigger_tick(),
        }
    }

    #[inline(always)]
    fn update_timer_trigger_tick(&mut self, id: usize) {
        let tick = match &self.timers[id] {
            TimerType::Base(t) => t.next_trigger_tick(),
            TimerType::Audio(t) => t.next_trigger_tick(),
        };
        self.timer_triggers[id] = tick;
    }

    #[inline(always)]
    pub fn audio_out(&self, n: usize) -> i16 {
        if n >= 8 {
            self.audio_settings[n - 8].output as i16
        } else {
            0
        }
    }
}

impl Default for Timers {
    fn default() -> Self {
        Timers::new()
    }
}
