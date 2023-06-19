mod atmosphere;
mod electrically_charged;
mod lightningrod;

pub mod prelude {
    pub use super::lightningrod::prelude::*;
    pub use crate::atmosphere::prelude::*;
    pub use crate::electrically_charged::ElectricallyCharged;
}

#[cfg(test)]
mod tests {
    use crate::electrically_charged::ElectricallyCharged;

    use super::prelude::*;

    #[test]
    fn test_lightningrod() {
        let mut rod = LightningRod::default();
        assert_eq!(rod.charge_status(), 0);

        rod.strike(0, 100);
        assert_eq!(rod.charge_status(), 100);
    }

    #[test]
    fn test_atmosphere() {
        let mut atmosphere = Atmosphere::new(10000, 100000);
        assert_eq!(atmosphere.charge_status(), 1000000000);
    }
}
