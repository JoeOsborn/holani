use hashbrown::HashMap;
use lazy_static::lazy_static;
use md5;

use super::lnx_header::LNXRotation;

pub fn check_no_intro(data: &[u8]) -> Result<(&'static str, LNXRotation), ()> {
    let md5 = format!("{:x}", md5::compute(data));
    let md5str = md5.as_str();

    if SET.contains_key(&md5str) {
        Ok(SET[&md5str])
    } else {
        Err(())
    }
}

lazy_static!{
    static ref SET: HashMap<&'static str, (&'static str , LNXRotation)> = {
        HashMap::from([
            ("b425941149874c6371c40e85cc5b6241", ("A.P.B. (USA, Europe)", LNXRotation::None)),
            ("8ce6c739d30d6c5ba197fcdd73d5ead5", ("Alien vs Predator (USA) (Proto) (1993-12-17)", LNXRotation::None)),
            ("49d6eeb3c983246ff4d7034497f7095c", ("Awesome Golf (USA, Europe)", LNXRotation::None)),
            ("8c9a72ddbb5559293862684ec67bfa92", ("Baseball Heroes (USA, Europe)", LNXRotation::None)),
            ("f19b95e4835c5fbc44b180dbd3f024fc", ("Basketbrawl (USA, Europe)", LNXRotation::None)),
            ("6e9bbff3c7b66d3ec0411ffbe0e41dfd", ("Batman Returns (USA, Europe)", LNXRotation::None)),
            ("fddecd756abe5eaf613ca1db31d51df7", ("Battle Wheels (USA, Europe)", LNXRotation::None)),
            ("d405ea54b77390b06222f9dac7cea827", ("Battlezone 2000 (USA, Europe)", LNXRotation::None)),
            ("87dff4f4d5e1e4a7132d19f94d4e9b3a", ("Bill & Ted's Excellent Adventure (USA, Europe)", LNXRotation::None)),
            ("c81abf2919effd525b83c2103b75e6ca", ("Block Out (USA, Europe)", LNXRotation::None)),
            ("7c3cb287de2d9f67ff342b4e42592732", ("Blue Lightning (USA, Europe) (Demo)", LNXRotation::None)),
            ("f7b4775771bc25d9053af5c69a8c8b2a", ("Blue Lightning (USA, Europe)", LNXRotation::None)),
            ("90841f8fed54862f8a8750ddf212eb84", ("Bubble Trouble (USA, Europe)", LNXRotation::None)),
            ("73e02bb77dcf857a6578f0e24b4ecb9e", ("California Games (USA, Europe)", LNXRotation::None)),
            ("13cb869b95c67c532efdd0e924e37299", ("Centipede (USA) (Proto)", LNXRotation::None)),
            ("85b33c5e5985ab041ecc44555a8cfb42", ("Checkered Flag (USA, Europe)", LNXRotation::None)),
            ("24611d1445ebd34ab79fb0ae996ff4e4", ("Chip's Challenge (USA, Europe)", LNXRotation::None)),
            ("b4acbd3c544a0d92cc8ad1380bf8a810", ("Crystal Mines II (USA, Europe)", LNXRotation::None)),
            ("f8b4debd68eb0d7c578242fa74e1c593", ("Daemonsgate (USA) (Proto)", LNXRotation::None)),
            ("f764b88c44afa8b5ebefb8eb2e4d7b97", ("Desert Strike - Return to the Gulf (USA, Europe)", LNXRotation::None)),
            ("9496be61fd0675553f05345c5fc2d15c", ("Dinolympics (USA, Europe)", LNXRotation::None)),
            ("8cd77bec912c9b4dcebd8a82dcf91a0b", ("Dirty Larry - Renegade Cop (USA, Europe)", LNXRotation::None)),
            ("0e91b7ed60bb47d569ba24df671ad3a3", ("Double Dragon (USA, Europe)", LNXRotation::None)),
            ("3ff35996887c2ff95e275085efbbbbed", ("Dracula the Undead (USA, Europe)", LNXRotation::None)),
            ("c49ca94a908db219224c6d5baa206ab6", ("Electrocop (USA, Europe)", LNXRotation::None)),
            ("32e726ab7941eb1e833dcc4cf348a060", ("European Soccer Challenge (USA, Europe)", LNXRotation::None)),
            ("858480bb97d86a52b1ca17dc390a8bdc", ("Eye of the Beholder (USA) (Proto)", LNXRotation::None)),
            ("7b3f49bda3162fac51a387905e6fb6f4", ("Eye of the Beholder (USA) (Unl)", LNXRotation::None)),
            ("69abd21c83390dae54630919c3c150d0", ("Fat Bobby (USA, Europe)", LNXRotation::None)),
            ("19eb0ca77284b5d91a63eb02f6962930", ("Fidelity Ultimate Chess Challenge, The (USA, Europe)", LNXRotation::None)),
            ("d5be9118bcb14243468001b08c4aa21a", ("Gates of Zendocon (USA, Europe)", LNXRotation::None)),
            ("8815ea087af1aa89b4f7b65bd3cb8534", ("Gauntlet - The Third Encounter (USA, Europe) (Beta) (1990-06-04)", LNXRotation::_270)),
            ("0b572c0dfb938849eeec39b2c9583547", ("Gauntlet - The Third Encounter (USA, Europe)", LNXRotation::_270)),
            ("eb9b5b2b6160e5f3015bc2a669d886b6", ("Gordo 106 (USA, Europe)", LNXRotation::None)),
            ("c265f0de8c5bd77db8f9cf5a1f7ab68f", ("Hard Drivin' (USA, Europe)", LNXRotation::None)),
            ("a08b8070ad613bdb2637162b3bf39574", ("Hockey (USA, Europe)", LNXRotation::None)),
            ("76a48869c14fbcf85588001a1327253d", ("Hydra (USA, Europe)", LNXRotation::None)),
            ("addc6c0ae7b535839815b8e7f7fd0f11", ("Ishido - The Way of Stones (USA, Europe)", LNXRotation::None)),
            ("087e6ed018ad0573bd7ce3a91d34f2c9", ("Jimmy Connors' Tennis (USA, Europe)", LNXRotation::None)),
            ("440462507cf5cffaa8d3d3a66f01ac6a", ("Joust (USA, Europe)", LNXRotation::None)),
            ("57043e8e79588c067118a4d5f307cd76", ("Klax (USA, Europe) (Beta)", LNXRotation::_270)),
            ("f96a0ddcc72c971226e8fdfd95607c88", ("Klax (USA, Europe)", LNXRotation::_270)),
            ("7e82db12a5749ab983e2f3c8bf4c0f6e", ("Krazy Ace - Miniature Golf (USA, Europe)", LNXRotation::None)),
            ("e5e42190918847b8c6056e78316ee91d", ("Kung Food (USA, Europe)", LNXRotation::None)),
            ("3cae85572df3b43f0220326bf4bb3c8b", ("Lemmings (USA, Europe)", LNXRotation::None)),
            ("7ee41edaef283459c9df93366c5da267", ("Lexis (USA)", LNXRotation::None)),
            ("96fd77f3527bc6f65977b99ab63d7f84", ("Lode Runner (USA) (Proto) (Unl)", LNXRotation::None)),
            ("8399c8fba48ba1a4389a96e75838dc49", ("Loopz (USA) (Proto)", LNXRotation::None)),
            ("05d28ab0e92b19147e7f5ea88c6efb6d", ("Lynx Casino (USA, Europe)", LNXRotation::None)),
            ("5d8fdfb15441cdfb8a1c66c243c486da", ("Lynx II Production Test Program (USA)", LNXRotation::None)),
            ("98c851c7ed924e1c7123c60e5164819e", ("Malibu Bikini Volleyball (USA, Europe) (Beta) (1993-05-11)", LNXRotation::None)),
            ("280344c8b073895ecce286d1b9d87d8b", ("Malibu Bikini Volleyball (USA, Europe)", LNXRotation::None)),
            ("194a3eeb876d2b74cc480f4d337d79b3", ("Marlboro Go! (Europe) (Proto)", LNXRotation::None)),
            ("192b6b764a3a1c7831e0a785fa4b5453", ("Ms. Pac-Man (USA, Europe)", LNXRotation::None)),
            ("276b9be28571189912f05d321fcb04ef", ("NFL Football (USA, Europe)", LNXRotation::_90)),
            ("7ec4063eb6c7c74600d6a16fb3a3bdbd", ("Ninja Gaiden (USA, Europe)", LNXRotation::None)),
            ("c9ed2a3bdefd6d5fdf67302d87b5cfb2", ("Ninja Gaiden III - The Ancient Ship of Doom (USA, Europe)", LNXRotation::None)),
            ("0a14754b351b4f11a1359e252b8eb992", ("Pac-Land (USA, Europe)", LNXRotation::None)),
            ("2a59d2ca6d6f07bc2791bf349e2778ee", ("Paperboy (USA, Europe)", LNXRotation::None)),
            ("29a248fbc87f477b49587581e29c1dc7", ("Pinball Jam (USA, Europe)", LNXRotation::None)),
            ("5565889a9a8817f99ec6dda322a70877", ("Pit-Fighter (USA, Europe)", LNXRotation::None)),
            ("b29414b8c81cc9ef28ef2f7a09d6d876", ("Power Factor (USA, Europe)", LNXRotation::None)),
            ("12e1eb0900402ef6de8b72dda5d22f47", ("QIX (USA, Europe)", LNXRotation::None)),
            ("abfd6ae93c31e8f59aa934ad922cb4dd", ("Raiden (USA) (Proto)", LNXRotation::_270)),
            ("e0cb426257761c3688a866332ed48340", ("Rampage (USA, Europe)", LNXRotation::None)),
            ("d8045ed542d5e58c779c884e0930e16c", ("Rampart (USA, Europe)", LNXRotation::None)),
            ("702e4d515d9f33698407b118c4cd373f", ("Road Riot 4WD (USA) (Proto 1)", LNXRotation::None)),
            ("8e0680d9d484749297bd7f4cfd5b7354", ("Road Riot 4WD (USA) (Proto 2)", LNXRotation::None)),
            ("9222e42a160924dee0c87a67fdbc48d0", ("Road Riot 4WD (USA) (Proto 3)", LNXRotation::None)),
            ("39617ebb81f3c1df27354c18571bd6c3", ("RoadBlasters (USA, Europe)", LNXRotation::None)),
            ("f9faa45e3c35e505249c3f9df801737b", ("Robo-Squash (USA, Europe)", LNXRotation::None)),
            ("60c1dfbf112bbb2a49cf16eddb191842", ("Robotron 2084 (USA, Europe)", LNXRotation::None)),
            ("ff6fff314446ab70dfecb21e2de4a2f6", ("Rygar (USA, Europe)", LNXRotation::None)),
            ("490f8063bbb299070c4aceab64195088", ("S.T.U.N. Runner (USA, Europe)", LNXRotation::None)),
            ("0cf228912d2f8eeb29aa215abf416f6d", ("Scrapyard Dog (USA, Europe)", LNXRotation::None)),
            ("f92d57198ef2da30dba63bdd7c15ff83", ("Shadow of the Beast (USA, Europe)", LNXRotation::None)),
            ("46634eb87e6380d4d10f7b80d177c1ff", ("Shanghai (USA, Europe)", LNXRotation::None)),
            ("8828c0042a1a397de67c4e49042161ff", ("Steel Talons (USA, Europe)", LNXRotation::None)),
            ("67dd69e6ffaf61bc85243d272d9ee9d9", ("Super Asteroids, Missile Command (USA, Europe)", LNXRotation::None)),
            ("6cd23cb37c4c4c34ef4e197468462f3f", ("Super Off-Road (USA, Europe)", LNXRotation::None)),
            ("a19802bd3a7e390daf7e2cbe5a81ed38", ("Super Skweek (USA, Europe)", LNXRotation::None)),
            ("4971dd8b47d3475dc8d31b5325c14459", ("Switchblade II (USA, Europe)", LNXRotation::None)),
            ("4581ac0418679567dce041c88cc97719", ("Todd's Adventures in Slime World (USA, Europe)", LNXRotation::None)),
            ("ec46311f47276e20cc43228c96d119a1", ("Toki (USA, Europe)", LNXRotation::None)),
            ("391dfaba9ab8b9b60e9d8ca2a73f5711", ("Tournament Cyberball (USA, Europe)", LNXRotation::None)),
            ("8caaaf56f95ee0bb610ca14af2d12a61", ("Turbo Sub (USA, Europe)", LNXRotation::None)),
            ("44d7c4ea6b8d930075f5b05c94b5f973", ("Viking Child (USA, Europe)", LNXRotation::None)),
            ("e7f118ac59f985ceea2ecfc0f17e9cc6", ("Warbirds (USA, Europe)", LNXRotation::None)),
            ("7fcf204013cbedf7eaaf682ff5f216ba", ("World Class Soccer (USA, Europe)", LNXRotation::None)),
            ("69fb597cb4db019c48fd2e0c1cc7b75c", ("Xenophobe (USA, Europe)", LNXRotation::None)),
            ("c145bfe904e5d56f479df44204b255da", ("Xybots (USA, Europe)", LNXRotation::None)),
            ("52997de9af205728a0e17ea3475f7ae2", ("Zaku (USA) (Beta) (Unl)", LNXRotation::None)),
            ("6b9d6872961b22de6b1b0cced65d8e3f", ("Zaku (USA) (Unl)", LNXRotation::None)),
            ("d008f41ec119e2c5c6a0782aebf148a8", ("Zarlor Mercenary (USA, Europe)", LNXRotation::None)),
        ])
    };
}