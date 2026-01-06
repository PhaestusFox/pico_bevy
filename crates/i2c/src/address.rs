use bevy::prelude::Component;

#[derive(Component)]
pub struct I2CAddress(pub u8);

impl I2CAddress {
    pub fn new(address: u8) -> Self {
        I2CAddress(address)
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}
