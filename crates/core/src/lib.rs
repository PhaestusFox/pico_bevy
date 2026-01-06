#![no_std]

/// The PicoBevy Core Plugin<br>
/// This plugin initializes the Raspberry Pi Pico<br>
/// Depending on the features enabled, it will add the corresponding peripheral Instances to the App world as no send resources
/// # Config
/// - Clocks: add a non send resource of type embassy_rp::clocks::ClockConfig to the App before adding the PicoCore plugin
/// # Features
/// - uart: adds UART peripheral instances
/// - spi: adds SPI peripheral instances
/// - i2c: adds I2C peripheral instances
/// - gpio: adds all GPIO Pin instances
pub struct PicoCore;

impl bevy::prelude::Plugin for PicoCore {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(feature = "defmt")]
        defmt::info!("Building PicoCore Plugin");
        let clock_config = app
            .world_mut()
            .remove_non_send_resource::<embassy_rp::clocks::ClockConfig>()
            .unwrap_or(embassy_rp::clocks::ClockConfig::crystal(12_000_000));
        #[cfg(feature = "defmt")]
        defmt::info!("ClockConfig obtained");

        let pac = embassy_rp::init(embassy_rp::config::Config::new(clock_config));
        #[cfg(feature = "defmt")]
        defmt::info!("Initialized embassy_rp");

        // Add your peripheral plugins here based on features
        #[cfg(feature = "uart")]
        {
            // add uart peripherals
            app.insert_non_send_resource(pac.UART0);
            app.insert_non_send_resource(pac.UART1);
        }
        #[cfg(feature = "spi")]
        {
            // add spi peripherals
            app.insert_non_send_resource(pac.SPI0);
            app.insert_non_send_resource(pac.SPI1);
        }
        #[cfg(feature = "i2c")]
        {
            // add i2c peripherals
            app.insert_non_send_resource(pac.I2C0);
            app.insert_non_send_resource(pac.I2C1);
        }
        #[cfg(feature = "gpio")]
        {
            // add gpio peripherals
            app.insert_non_send_resource(pac.PIN_0);
            app.insert_non_send_resource(pac.PIN_1);
            app.insert_non_send_resource(pac.PIN_2);
            app.insert_non_send_resource(pac.PIN_3);
            app.insert_non_send_resource(pac.PIN_4);
            app.insert_non_send_resource(pac.PIN_5);
            app.insert_non_send_resource(pac.PIN_6);
            app.insert_non_send_resource(pac.PIN_7);
            app.insert_non_send_resource(pac.PIN_8);
            app.insert_non_send_resource(pac.PIN_9);
            app.insert_non_send_resource(pac.PIN_10);
            app.insert_non_send_resource(pac.PIN_11);
            app.insert_non_send_resource(pac.PIN_12);
            app.insert_non_send_resource(pac.PIN_13);
            app.insert_non_send_resource(pac.PIN_14);
            app.insert_non_send_resource(pac.PIN_15);
            app.insert_non_send_resource(pac.PIN_16);
            app.insert_non_send_resource(pac.PIN_17);
            app.insert_non_send_resource(pac.PIN_18);
            app.insert_non_send_resource(pac.PIN_19);
            app.insert_non_send_resource(pac.PIN_20);
            app.insert_non_send_resource(pac.PIN_21);
            app.insert_non_send_resource(pac.PIN_22);
            app.insert_non_send_resource(pac.PIN_23);
            app.insert_non_send_resource(pac.PIN_24);
            app.insert_non_send_resource(pac.PIN_25);
            app.insert_non_send_resource(pac.PIN_26);
            app.insert_non_send_resource(pac.PIN_27);
            app.insert_non_send_resource(pac.PIN_28);
            app.insert_non_send_resource(pac.PIN_29);
        }
        #[cfg(feature = "watchdog")]
        {
            // add watchdog peripheral
            app.insert_non_send_resource(pac.WATCHDOG);
        }
        #[cfg(feature = "rtc")]
        {
            // add rtc peripheral
            app.insert_non_send_resource(pac.RTC);
        }
    }
}

#[cfg(feature = "gpio")]
pub mod gpio;

#[cfg(feature = "gpio")]
pub use gpio::*;

#[derive(bevy::prelude::Component)]
pub struct UseBus<P>(core::marker::PhantomData<P>);

impl<P> UseBus<P> {
    // Have new but no default so its explicit to call and dont get accidental behavior for impl Default
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}
