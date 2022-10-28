use std::sync::{Arc, Mutex};

pub mod prelude {
    pub use crate::lightningrod::LightningRod;
}

#[derive(Clone)]
pub struct LightningRod {
    pub static_charge_exhaust: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>,
    pub pole: Arc<Vec<Arc<Mutex<Option<u8>>>>>,
}


impl LightningRod {
    pub fn new(exhaust_handle: Arc<Mutex<Vec<Arc<Mutex<Option<u8>>>>>>) -> LightningRod {
        LightningRod {
            static_charge_exhaust: exhaust_handle,
            pole: Arc::new(vec![Arc::new(Mutex::new(None)); 100]),
        }
    }

    // Could start a trait instead of pub, but not a well enough defined interface.
    pub fn strike(&mut self, index: usize, charge: u8) {
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
