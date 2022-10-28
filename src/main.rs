use spark_generator::prelude::*;

fn main() {
    let mut atmosphere = ThunderboltCatcher::default();
    atmosphere.charge(std::time::Duration::from_secs(3));
}
