use bevy::app::Plugin;

use embassy_rp::peripherals::I2C0;

use crate::I2CPeripheral;

impl<P: I2CPeripheral + Send + Sync + 'static> Plugin for I2CPlugin<P> {
    fn build(&self, app: &mut bevy::app::App) {
        #[cfg(feature = "defmt")]
        defmt::info!(
            "Building PicoI2CPlugin for {} on SDA({}) SCL({})",
            P::NAME,
            self.sda,
            self.scl
        );
        if !app.is_plugin_added::<pico_bevy_core::PicoCore>() {
            #[cfg(feature = "defmt")]
            defmt::error!("PicoCore plugin must be added before I2CPlugin");
            return;
        }
        let Ok(i2c) = P::get_i2c(app.world_mut(), self.sda, self.scl, self.config) else {
            #[cfg(feature = "defmt")]
            defmt::error!("Failed to create {} instance", P::NAME);
            return;
        };
        app.insert_resource(super::I2CBus::<P>::new(i2c));
        #[cfg(feature = "defmt")]
        defmt::info!("{} peripheral added", P::NAME);
    }
}

pub struct I2CPlugin<I: I2CPeripheral> {
    pub(crate) sda: I::SDAPins,
    pub(crate) scl: I::SCLPins,
    pub(crate) config: embassy_rp::i2c::Config,
}

impl<I: I2CPeripheral> I2CPlugin<I> {
    pub fn with_config(mut self, config: embassy_rp::i2c::Config) -> Self {
        self.config = config;
        self
    }
}

impl Default for I2CPlugin<I2C0> {
    fn default() -> Self {
        Self::i2c0(
            crate::i2c0::SDAPins::default(),
            crate::i2c0::SCLPins::default(),
        )
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum MakeI2CError {
    PeripheralTaken,
    SDATaken,
    SCLTaken,
}
