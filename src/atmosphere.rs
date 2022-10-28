use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::atmosphere;
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
    pub fn new() -> Self {

        Self {
            charge: Arc::new(Mutex::new(vec![Arc::new(Mutex::new(None)); 10000])),
        }
    }
}

pub struct ThunderboltCatcher {
    charge_collector: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>,
    rods: Arc<Mutex<Vec<LightningRod>>>,
    atmosphere: Arc<Mutex<Atmosphere>>,
}

impl ThunderboltCatcher {
    pub fn new() -> Self {
        // 100 poles, 10,000 arbirtrary charges. Ergo, 1,000,000% efficiency. :P

        let charge_collector = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(None)); 100])); 

        Self {

            charge_collector: charge_collector.clone(),
            rods: Arc::new(Mutex::new(vec![LightningRod::new(charge_collector.clone()); 100])),
            atmosphere: Arc::new(Mutex::new(Atmosphere::new())),
        }
    }
}

pub trait ThunderboltThrower {
    fn new() -> Self;

    fn charge(&mut self);

    fn generate(&mut self, run_duration: Duration);
    fn collect_charge(&mut self);
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
        #[cfg(feature = "performant_speech")] println!("Charging from the atmosphere for {:?}", run_duration);
        let atmosphere = self.atmosphere.clone(); // Lock the atmosphere.

        // Be striked by lightning, in parallel!
        let rods = self.rods.clone();
        let mut handles = vec![]; // We'll store the handles here so we can join them later

        for rod in (*rods).lock().unwrap().iter_mut() {
            let mut rod = rod.clone();
            let charge = atmosphere.lock().unwrap().charge.clone();
            let handle = thread::spawn(move || {
                let mut lifetime = Duration::from_millis(rand::random::<u64>() % 1000);
                let duration = Duration::from_micros((rand::random::<u64>() % 10) + 1);

                loop {
                    // Strike several times, randomly, taking a random charge from the atmosphere, it's scientific!
                    // First lets get the charge from the atmosphere, if there is any. :P
                    for c in charge.lock().unwrap().iter_mut() {
                        if let Ok(mut c) = c.lock() {
                            if let Some(chg) = *c {
                                rod.strike(rand::random::<usize>() % rod.pole.len(), (chg % 255) as u8);
                                *c = Some(chg - (chg % 255));
                                break;
                            }
                        }
                    }

                    // Because everything must end, eventually, even Maxwell's little daemons.
                    if lifetime < duration {
                        break;
                    } else {
                        lifetime -= duration;
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

    fn collect_charge(&mut self) {
        #[cfg(feature = "performant_speech")] println!("Collecting charge from the atmosphere");
        let atmosphere = self.atmosphere.lock().unwrap(); // Lock the atmosphere.

        // Disperse the charge_collector back to the atmosphere.
        let charge_collector = self.charge_collector.lock().unwrap();
        let charge = atmosphere.charge.lock().unwrap();
        // We'll do this by subtracting 1 from each slot in charge_collector and dispersing it into the slots in charge.
        for (i, charge) in charge.iter().enumerate() {
            let mut charge = charge.lock().unwrap();
            // grab a random charge collector, there's only 100 of them, while there are many more charges.
            let mut charge_collector = charge_collector[rand::random::<usize>() % charge_collector.len()].lock().unwrap();
            
            match *charge_collector {
                Some(ref mut source) => {
                    if *source > 0 {
                        *source -= 1;
                        #[cfg(feature = "talking_electricity")] println!("Dispersing {}GeV from pole {} to charge slot {}", c, i, i);
                        match *charge {
                            Some(ref mut dest) => *dest += 1,
                            None => *charge = Some(1),
                        }
                    }
                }
                None => (),
            }
        }
    }
}