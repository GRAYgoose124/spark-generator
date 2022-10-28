use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


#[derive(Clone)]
struct LightningRod {
    static_charge_exhaust: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>,
    pole: Arc<Vec<Arc<Mutex<Option<u8>>>>>,
}

impl LightningRod {
    fn new(exhaust_handle: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>) -> LightningRod {
        LightningRod {
            static_charge_exhaust: exhaust_handle,
            pole: Arc::new(vec![Arc::new(Mutex::new(None)); 100]),
        }
    }

    fn strike(&mut self, index: usize, charge: u8) {
        // We can do this and it's cheap, because it's an Arc.
        let pole = self.pole.clone();

        // Lets use the lock to accumulate the charge, if locked, search for the next unlocked indice.
        match pole[index].lock() {
            // If okay, strike the pole here, adding the charge.
            Ok(mut pole) => {
                match *pole {
                    Some(ref mut c) => {
                        // Discharge instead of overflow on u8.
                        if ((*c as usize) + charge as usize) >= u8::max_value().into() {
                            #[cfg(feature = "talking_electricity")] println!("Dissipating {}GeV from pole {} magically!", c, index);

                            *c = 0;
                            // Prepare the charge to go back into the atmostphere, we'll call this static lmfao.
                            self.static_charge_exhaust.lock().unwrap().push(Arc::new(Mutex::new(Some(charge))));
                        } else {
                            #[cfg(feature = "talking_electricity")] println!("Striking with a gusto of {}GeV to pole {}", charge, index);
                            *c += charge;
                        }
                        
                    },
                    None => *pole = Some(charge),
                }
            }
            // If not okay, search for the next unlocked indice by recursively calling strike.
            Err(_) => {
                let mut i = index + 1;
                if i >= pole.len() {
                    i = 0;
                }

                self.strike(i, charge);
                // Strike the other side of the pole, lets just call this bifurcated lighting for now. :P
                self.strike(self.pole.len() - i, charge); 
            }
        };
    }
}

struct Atmosphere {
    charge_collector: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>,
    charge: Arc<Mutex<Vec<Arc<Mutex<Option<u64>>>>>>,
    rods: Arc<Mutex<Vec<LightningRod>>>,
}

impl Atmosphere {
    fn new() -> Self {
        // 100 poles, 10,000 arbirtrary charges. Ergo, 1,000,000% efficiency. :P
        let charge_collector = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(None)); 100])); 

        Self {
            charge_collector: charge_collector.clone(),
            charge: Arc::new(Mutex::new(vec![Arc::new(Mutex::new(None)); 10000])),
            rods: Arc::new(Mutex::new(vec![LightningRod::new(charge_collector.clone()); 100])),
        }
    }

    fn charge_generator(&mut self) {
        // Collect charge from the atmosphere, it's parallel!
        let rods = self.rods.clone();
        let mut handles = vec![]; // We'll store the handles here so we can join them later

        for rod in (*rods).lock().unwrap().iter_mut() {
            let mut rod = rod.clone();
            let handle = thread::spawn(move || {
                let mut lifetime = Duration::from_millis(rand::random::<u64>() % 1000);
                let duration = Duration::from_micros((rand::random::<u64>() % 1000) + 1);

                loop {
                    // Strike several times, randomly, with a random charge, it's scientific!
                    for i in 0..(rand::random::<u64>() % 100) + 1 {
                        rod.strike(i as usize, rand::random::<u8>());
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
        for handle in handles {
            handle.join().unwrap();
        }

        // Disperse the charge_collector back to the atmosphere.
        let charge_collector = self.charge_collector.lock().unwrap();
        let charge = self.charge.lock().unwrap();
        // We'll do this by subtracting 1 from each slot in charge_collector and dispersing it into the slots in charge.
        for (i, charge) in charge.iter().enumerate() {
            let mut charge = charge.lock().unwrap();
            // grab a random charge collector, there's only 100 of them, while there are many more charges.
            let mut charge_collector = charge_collector[rand::random::<usize>() % charge_collector.len()].lock().unwrap();
            
            match *charge_collector {
                Some(ref mut c) => {
                    if *c > 0 {
                        *c -= 1;
                        #[cfg(feature = "talking_electricity")] println!("Dispersing {}GeV from pole {} to charge slot {}", c, i, i);
                        match *charge {
                            Some(ref mut c) => *c += 1,
                            None => *charge = Some(1),
                        }
                    }
                }
                None => (),
            }
        }
    }
}

fn main() {
    let mut atmosphere = Atmosphere::new();
    atmosphere.charge_generator();
}
