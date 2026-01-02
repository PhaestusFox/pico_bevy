use bevy::app::Plugin;

use embassy_rp::peripherals::UART0;

use crate::UArtPeripheral;

impl<P: UArtPeripheral + Send + Sync + 'static> Plugin for UArtPlugin<P> {
    fn build(&self, app: &mut bevy::app::App) {
        #[cfg(feature = "defmt")]
        defmt::info!(
            "Building PicoUArtPlugin for {} on Tx({}) Rx({})",
            P::NAME,
            self.tx,
            self.rx
        );
        if !app.is_plugin_added::<pico_bevy_core::PicoCore>() {
            #[cfg(feature = "defmt")]
            defmt::error!("PicoCore plugin must be added before UARTPlugin");
            return;
        }
        let Ok(uart) = P::get_uart(app.world_mut(), self.tx, self.rx, self.config) else {
            #[cfg(feature = "defmt")]
            defmt::error!("Failed to create {} instance", P::NAME);
            return;
        };
        app.insert_resource(super::UArtBus::<P>::new(uart));
        #[cfg(feature = "defmt")]
        defmt::info!("{} peripheral added", P::NAME);
    }
}

pub struct UArtPlugin<I: UArtPeripheral> {
    pub(crate) tx: I::TxPins,
    pub(crate) rx: I::RxPins,
    pub(crate) config: embassy_rp::uart::Config,
}

impl<I: UArtPeripheral> UArtPlugin<I> {
    pub fn with_config(mut self, config: embassy_rp::uart::Config) -> Self {
        self.config = config;
        self
    }
}

impl Default for UArtPlugin<UART0> {
    fn default() -> Self {
        Self::uart0(super::uart0::TxPins::Gpio0, super::uart0::RxPins::Gpio1)
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum MakeUArtError {
    PeripheralTaken,
    TxTaken,
    RxTaken,
}
