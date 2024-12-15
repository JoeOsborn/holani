use alloc::fmt;
use log::trace;

use super::*;
use core::num::NonZeroU8;
#[derive(Clone, Serialize, Deserialize)]
pub struct AudioChannelTimer {
    id: u8,
    backup: u8,
    control_a: u8,
    count: u8,
    control_b: u8,
    clock_ticks: Option<u32>,
    next_trigger_tick: u64,
    linked_timer: Option<NonZeroU8>,
    is_linked: bool,
    count_enabled: bool,
    reload_enabled: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct AudioSettings {
    pub volume: u8,
    pub feedback: u8,
    pub shift_register: u8,
    pub output: i8,
    pub disabled: bool,
}

impl AudioChannelTimer {
    pub fn new(id: u8, linked_timer: Option<NonZeroU8>) -> Self {
        Self {
            id,
            backup: 0,
            control_a: 0,
            count: 0,
            control_b: 0,
            clock_ticks: None,
            next_trigger_tick: u64::MAX,
            linked_timer,
            is_linked: false,
            count_enabled: false,
            reload_enabled: false,
        }
    }

    pub fn linked_timer(&self) -> Option<NonZeroU8> {
        self.linked_timer
    }

    #[allow(dead_code)]
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn backup(&self) -> u8 {
        self.backup
    }
    pub fn set_backup(&mut self, v: u8) {
        self.backup = v;
    }

    pub fn control_a(&self) -> u8 {
        self.control_a
    }

    pub fn control_b(&self) -> u8 {
        self.control_b
    }

    pub fn reset_timer_done(&mut self) {
        self.control_b &= !CTRLB_TIMER_DONE_BIT;
    }

    pub fn set_control_a(&mut self, value: u8, current_tick: u64) {
        trace!("AudioTimer #{} ctrl_a = {}.", self.id, value);
        self.control_a = value;
        self.clock_ticks = match self.period() {
            7 => None,
            v => Some(TIMER_TICKS_COUNT as u32 * u32::pow(2, v as u32)),
        };
        if value & CTRLA_RESET_DONE_BIT != 0 {
            self.reset_timer_done();
        }
        self.is_linked = self.period() == 7;
        self.count_enabled = value & CTRLA_ENABLE_COUNT_BIT != 0;
        self.reload_enabled = value & CTRLA_ENABLE_RELOAD_BIT != 0;

        if !self.is_linked && self.count_enabled {
            self.next_trigger_tick = current_tick + self.clock_ticks.unwrap() as u64;
        } else {
            self.next_trigger_tick = u64::MAX;
        }
    }

    pub fn set_control_b(&mut self, value: u8) {
        trace!("AudioTimer #{} ctrl_b = {}.", self.id, value);
        self.control_b = value;
    }

    pub fn set_count(&mut self, value: u8, current_tick: u64) {
        trace!("AudioTimer #{} count = {}.", self.id, value);
        self.count = value;
        if !self.is_linked && self.count_enabled && value != 0 {
            self.next_trigger_tick = current_tick + self.clock_ticks.unwrap() as u64;
            trace!(
                "AudioTimer #{} next trigger @ {}",
                self.id,
                self.next_trigger_tick
            );
        }
    }

    fn period(&self) -> u8 {
        self.control_a() & CTRLA_PERIOD_BIT
    }

    fn interrupt_enabled(&self) -> bool {
        self.control_a() & CTRLA_INTERRUPT_BIT != 0
    }

    fn integrate(&self) -> bool {
        self.control_a & 0b00100000 != 0
    }

    pub fn is_linked(&self) -> bool {
        self.is_linked
    }
    pub fn count(&self) -> u8 {
        self.count
    }

    fn count_down(&mut self, settings: &mut AudioSettings) -> (bool, u8) {
        // trace!("AudioTimer #{} count down.", self.id);
        self.control_b &= !CTRLB_BORROW_OUT_BIT;
        self.control_b |= CTRLB_BORROW_IN_BIT;
        match self.count.cmp(&0) {
            core::cmp::Ordering::Greater => self.count -= 1,
            core::cmp::Ordering::Equal => {
                if self.reload_enabled {
                    trace!("AudioTimer #{} reload 0x{:02x}.", self.id, self.backup);
                    self.count = self.backup;
                } else {
                    self.next_trigger_tick = u64::MAX;
                }

                self.done(settings);

                return (true, 0);
            }
            _ => (),
        }
        (false, 0)
    }

    pub fn tick_linked(&mut self, settings: &mut AudioSettings) -> (bool, u8) {
        if !self.is_linked {
            return (false, 0);
        }

        if self.count_enabled && !settings.disabled {
            return self.count_down(settings);
        }
        (false, 0)
    }

    pub fn tick(&mut self, current_tick: u64, settings: &mut AudioSettings) -> (bool, u8) {
        self.control_b &= !CTRLB_BORROW_IN_BIT;

        if !self.count_enabled || settings.disabled {
            self.next_trigger_tick = u64::MAX;
            return (false, 0);
        }

        self.next_trigger_tick = current_tick + self.clock_ticks.unwrap() as u64;

        self.count_down(settings)
    }

    fn done(&mut self, settings: &mut AudioSettings) {
        trace!("AudioTimer #{} done.", self.id);

        self.control_b |= CTRLB_TIMER_DONE_BIT | CTRLB_BORROW_OUT_BIT;

        /* "
        The inversion of the output of the gate is used as the data input to the shift register. [...]
        This same inverted output is taken from the exclusive or gate and sent to the waveshape selector. [...]
        The repeat period is programmed by selecting the initial value in the shift register (set shifter) and by picking which feedback taps are connected.
        " */
        let taps = self.audio_feedback_taps(settings);
        let shift_reg = self.audio_shift_register(settings);
        let par = (taps & shift_reg).count_ones() as u16 & 1 ^ 1;

        self.set_audio_shift_register(settings, (shift_reg << 1) | par);

        let volume = settings.volume as i8;

        settings.output = match self.integrate() {
            // "In integrate mode, instead of sending the volume register directly to the DAC it instead adds the volume register (or it's 2's complement) to a running total that is then sent to the DAC."
            // "In integrate mode, shift reg 0 = 1: add volume register to output."
            // "In integrate mode, shift reg 0 = 0: subtract volume register from output."
            true => match par {
                0 => settings.output.saturating_add(volume),
                _ => settings.output.saturating_sub(volume),
            },
            // "In normal nonintegrate mode, the bit selects either the value in the volume register or its 2's complement and sends it to the output DAC."
            // "In normal mode, shift reg 0 = 1: contains value of volume register."
            // "In normal mode, shift reg 0 = 0: contains 2's complement of volume register."
            false => match par {
                0 => volume,
                _ => -volume,
            },
        };

        trace!(
            "AudioTimer #{} output:0x{:02x} {:?}.",
            self.id,
            settings.output as u8,
            self
        );
    }

    pub fn set_shift_register(&mut self, settings: &mut AudioSettings, shift_register: u8) {
        settings.shift_register = shift_register;
    }

    #[allow(dead_code)]
    fn set_audio_feedback(&mut self, settings: &mut AudioSettings, feedback: u16) {
        trace!("AudioTimer #{} feedback = {}.", self.id, feedback);
        self.control_a &= !0b10000000;
        self.control_a |= (feedback as u8) & 0b10000000; // B7=feedback bit 7
        settings.feedback = (feedback as u8) & 0b00111111;
        settings.feedback |= ((feedback & 0b00001100_00000000) >> 4) as u8; // B7= feedback bit 11, B6=feedback bit 10
    }

    pub fn set_audio_shift_register(&mut self, settings: &mut AudioSettings, value: u16) {
        trace!("AudioTimer #{} shift register = {}.", self.id, value);
        settings.shift_register = value as u8;
        self.control_b &= !0b11110000; // B7=shift register bit 11, B6=shift register bit 10, B5=shift register bit 9, B4=shift register bit 8
        self.control_b |= ((value & 0b00001111_00000000) >> 4) as u8;
    }

    pub fn audio_shift_register(&self, settings: &AudioSettings) -> u16 {
        settings.shift_register as u16 | ((self.control_b as u16 & 0b11110000) << 4)
    }

    pub fn audio_feedback_taps(&self, settings: &AudioSettings) -> u16 {
        let mut fb = settings.feedback as u16 & 0b00111111;
        fb |= (settings.feedback as u16 & 0b11000000) << 4;
        fb |= (self.control_a & 0b10000000) as u16;
        fb
    }

    pub fn next_trigger_tick(&self) -> u64 {
        self.next_trigger_tick
    }
}

impl fmt::Debug for AudioChannelTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Timer #:{}, backup:{}, period:{}, int:{} reload:{}, count:{}, islinked:{}",
            self.id,
            self.backup,
            self.period(),
            self.interrupt_enabled(),
            self.reload_enabled,
            self.count_enabled,
            self.is_linked()
        )
    }
}
