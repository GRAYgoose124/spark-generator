use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::prelude::LightningRod;

pub mod prelude {
    pub use crate::atmosphere::Atmosphere;
    pub use crate::atmosphere::ThunderboltCatcher;

    pub use crate::atmosphere::ThunderboltThrower;
}

pub struct Atmosphere {
    charge: Arc<Mutex<Vec<Arc<Mutex<Option<u64>>>>>>,
}

impl Atmosphere {
    pub fn new(slots: usize, charge: u64) -> Self {
        Self {
            charge: Arc::new(Mutex::new(vec![Arc::new(Mutex::new(Some(charge))); slots])),
        }
    }
}

impl Default for Atmosphere {
    fn default() -> Self {
        Self::new(100000, 100000)
    }
}

pub struct ThunderboltCatcher {
    charge_collector: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>,
    rods: Arc<Mutex<Vec<LightningRod>>>,
    atmosphere: Arc<Mutex<Atmosphere>>,
}

impl Default for ThunderboltCatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ThunderboltCatcher {
    pub fn new() -> Self {
        // 100 poles, 10,000 arbirtrary charges. Ergo, 1,000,000% efficiency. :P

        let charge_collector = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(None)); 100]));

        Self {
            charge_collector: charge_collector.clone(),
            rods: Arc::new(Mutex::new(vec![
                LightningRod::new(charge_collector.clone());
                100
            ])),
            atmosphere: Arc::new(Mutex::new(Atmosphere::default())),
        }
    }

    fn collect_charge(&mut self) {
        #[cfg(feature = "performant_speech")]
        println!("Collecting charge from the atmosphere");
        let atmosphere = self.atmosphere.lock().unwrap(); // Lock the atmosphere.

        // Disperse the charge_collector back to the atmosphere.
        let charge_collector = self.charge_collector.lock().unwrap();
        let charge = atmosphere.charge.lock().unwrap();
        // We'll do this by subtracting 1 from each slot in charge_collector and dispersing it into the slots in charge.
        for charge in charge.iter() {
            let mut charge = charge.lock().unwrap();
            // grab a random charge collector, there's only 100 of them, while there are many more charges.
            let charge_slot = rand::random::<usize>() % charge_collector.len();
            let mut charge_collector = charge_collector[charge_slot].lock().unwrap();

            match *charge_collector {
                Some(ref mut source) => {
                    if *source > 0 {
                        #[cfg(feature = "talking_electricity")]
                        println!(
                            "Dispersing {}GeV from charge slot {} to atmosphere node {}",
                            source, charge_slot, i
                        );
                        match *charge {
                            Some(ref mut dest) => {
                                // Get random amount from source to dest.
                                let value = num_traits::clamp_max(rand::random::<u8>(), *source);
                                *source -= value; // Clamp the source to the dest.
                                *dest += value as u64;
                            }
                            None => *charge = Some(1),
                        }
                    }
                }
                None => (),
            }
        }
    }
}

pub trait ThunderboltThrower {
    fn new() -> Self;

    fn charge(&mut self);

    fn generate(&mut self, run_duration: Duration);
}

impl ThunderboltThrower for ThunderboltCatcher {
    fn new() -> Self {
        Self::new()
    }

    fn charge(&mut self) {
        self.generate(Duration::from_secs(1));
        self.collect_charge();
    }

    fn generate(&mut self, run_duration: Duration) {
        #[cfg(feature = "performant_speech")]
        println!("Charging from the atmosphere for {:?}", run_duration);
        let atmosphere = self.atmosphere.clone(); // Lock the atmosphere.

        // Be striked by lightning, in parallel!
        let rods = self.rods.clone();
        let mut handles = vec![]; // We'll store the handles here so we can join them later

        for rod in (*rods).lock().unwrap().iter_mut() {
            let mut rod = rod.clone();
            let charge = atmosphere.lock().unwrap().charge.clone();
            let handle = thread::spawn(move || {
                let mut living_time = Instant::now();
                let duration = Duration::from_micros((rand::random::<u64>() % 10) + 1);

                loop {
                    // Strike several times, randomly, taking a random charge from the atmosphere, it's scientific!
                    // First lets get the charge from the atmosphere, if there is any. :P
                    for c in charge.lock().unwrap().iter_mut() {
                        #[cfg(feature = "no_delay_generation_exit")]
                        if living_time.elapsed() > run_duration {
                            break;
                        }

                        if let Ok(mut c) = c.lock() {
                            if let Some(chg) = *c {
                                // Clamp the charge to u8::max_value() so we don't overflow.
                                // We'll call this the current limit. :P
                                let limit_charge =
                                    num_traits::clamp_max(chg, u8::max_value() as u64 - 2);
                                let actual_charge = (rand::random::<u8>()) % limit_charge as u8; // Get a random charge from the atmosphere. :P

                                rod.strike(
                                    rand::random::<usize>() % rod.pole.len(),
                                    actual_charge as u8,
                                );
                                *c = Some(chg - actual_charge as u64);
                                break;
                            }
                        }
                    }
                    // Because everything must end, eventually, even Maxwell's little daemons.
                    if living_time.elapsed() > run_duration {
                        break;
                    } else {
                        living_time += duration;
                        thread::sleep(duration);
                    }
                }
            });
            // You really shouldn't touch active lightning rods, but we do.
            handles.push(handle);
        }
        // Unite the handles, it's a magical gesture.
        //We'll just pretend all the lightning rods are in a circuit.
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
