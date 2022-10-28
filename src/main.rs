use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


#[derive(Clone)]
struct LightningRod {
    exhaust: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>,
    pole: Arc<Vec<Arc<Mutex<Option<u8>>>>>,
}

impl LightningRod {
    fn new(exhaust_handle: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>) -> LightningRod {
        LightningRod {
            exhaust: exhaust_handle,
            pole: Arc::new(vec![Arc::new(Mutex::new(None)); 100]),
        }
    }

    fn strike(&mut self, index: usize, charge: u8) {
        // We can do this because it's an Arc.
        let pole = self.pole.clone();

        // Lets use the lock to accumulate the charge, if locked, search for the next unlocked indice.
        match pole[index].lock() {
            // If okay, strike the pole here, adding the charge.
            Ok(mut pole) => {
                match *pole {
                    Some(ref mut c) => {
                        // Discharge instead of overflow on u8.
                        if ((*c as usize) + charge as usize) >= u8::max_value().into() {
                            #[cfg(feature = "talking_electricity")] println!("Discharging {} from pole {}", c, index);

                            *c = 0;
                            // Put the charge back into the pool.
                            self.exhaust.lock().unwrap().push(Arc::new(Mutex::new(Some(charge))));
                        } else {
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
    charge: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>,
    rods: Arc<Mutex<Vec<LightningRod>>>,
}

impl Atmosphere {
    fn new() -> Self {
        // 100 poles, 100 charges. 100% efficiency. :P
        let charge_collector = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(None)); 100])); 

        Self {
            charge_collector: charge_collector.clone(),
            charge: Arc::new(Mutex::new(vec![Arc::new(Mutex::new(None)); 100])),
            rods: Arc::new(Mutex::new(vec![LightningRod::new(charge_collector.clone()); 100])),
        }
    }

    fn charge_generator(&mut self) {
        // Lets use threads to strike the rods in parallel
        let rods = self.rods.clone();

        let mut handles = vec![]; // We'll store the handles here so we can join them later
        for rod in (*rods).lock().unwrap().iter_mut() {
            let mut rod = rod.clone();
            let handle = thread::spawn(move || {
                let mut lifetime = Duration::from_millis(rand::random::<u64>() % 1000);
                let duration = Duration::from_micros((rand::random::<u64>() % 1000) + 1);

                loop {
                    // Strike several times, randomly, with a random charge.
                    for i in 0..(rand::random::<u64>() % 100) + 1 {
                        rod.strike(i as usize, rand::random::<u8>());
                    }

                    if lifetime < duration {
                        break;
                    } else {
                        lifetime -= duration;
                        thread::sleep(duration);
                    }
                }                
            });
            handles.push(handle);
        }
        // Join the handles
        for handle in handles {
            handle.join().unwrap();
        }

        // Disperse the charge_collector back to the atmosphere.
        let mut charge_collector = self.charge_collector.lock().unwrap();
        let charge = self.charge.lock().unwrap();
        for (i, c) in charge_collector.iter_mut().enumerate() {
            match *c.lock().unwrap() {
                Some(ref mut c) => {
                    match *charge[i].lock().unwrap() {
                        Some(ref mut ch) => {
                            // Discharge instead of overflow on u8.
                            if *ch == u8::max_value() {
                                *ch = 0;
                                // Put the charge back into the pool.
                                *c += 1;
                            } else {
                                *ch += 1;
                            }
                        },
                        None => *charge[i].lock().unwrap() = Some(1),
                    }
                },
                None => (),
            }
        }
    }
}

fn main() {
    let mut atmosphere = Atmosphere::new();
    atmosphere.charge_generator();
}
