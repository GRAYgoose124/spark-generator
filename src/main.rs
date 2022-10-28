use num_traits::ops::overflowing::OverflowingAdd;
use spark_generator::prelude::*;

fn main() {
    let mut tc = ThunderboltCatcher::default();
    println!("Atmosphere charge before: {}", tc.atmosphere.lock().unwrap().charge_status()); // 1000000000000

    tc.generate(std::time::Duration::from_secs(3));
    let active_charge =tc.charge_collector
    .lock()
    .unwrap()
    .iter()
    .fold(0, |acc, c| {
        if let Some(c) = c.lock().unwrap().clone() {
            acc.overflowing_add(&c).0
        } else {
            acc
        }
    });

    println!("Atmosphere charge after: {}", tc.atmosphere.lock().unwrap().charge_status()); // 1000000000000
    println!("Active charge: {}", active_charge); // 0

    tc.disperse_collected();
    let dispersed_remaining =tc.charge_collector
    .lock()
    .unwrap()
    .iter()
    .fold(0, |acc, c| {
        if let Some(c) = c.lock().unwrap().clone() {
            acc.overflowing_add(&c).0
        } else {
            acc
        }
    });
    println!("Atmosphere charge after dispersal: {}", tc.atmosphere.lock().unwrap().charge_status()); // 1000000000000
    println!("Remaining charge after dispersal: {}", dispersed_remaining); // 0

    println!("Well, that's wasteful!")
}
