#![no_std]
mod plugin;

use bevy::{
    ecs::world::World,
    prelude::{Deref, DerefMut},
};
use embassy_rp::Peri;
use pico_bevy_core::gpio::PicoPin;
pub use plugin::{I2CPlugin, MakeI2CError};

pub type I2CError = embassy_rp::i2c::Error;

pub use address::I2CAddress;
pub use bus::{I2CBus, UseBus};

mod address;
mod bus;

pub trait I2CPeripheral: embassy_rp::i2c::Instance + Send + Sync + 'static {
    #[cfg(feature = "defmt")]
    type SDAPins: Send + Sync + Copy + 'static + defmt::Format;
    #[cfg(not(feature = "defmt"))]
    type SDAPins: Send + Sync + Copy + 'static;
    #[cfg(feature = "defmt")]
    type SCLPins: Send + Sync + Copy + 'static + defmt::Format;
    #[cfg(not(feature = "defmt"))]
    type SCLPins: Send + Sync + Copy + 'static;
    const NAME: &'static str;
    fn get_i2c(
        world: &mut World,
        sda_pin: Self::SDAPins,
        scl_pin: Self::SCLPins,
        config: embassy_rp::i2c::Config,
    ) -> Result<embassy_rp::i2c::I2c<'static, Self, embassy_rp::i2c::Blocking>, MakeI2CError>;
    fn get_pin<T: PicoPin>(world: &mut World) -> Option<Peri<'static, T::EmbassyType>> {
        world.remove_non_send_resource::<Peri<'static, T::EmbassyType>>()
    }
    fn make_i2c<
        SDA: PicoPin<EmbassyType: embassy_rp::i2c::SdaPin<Self>> + 'static,
        SCL: PicoPin<EmbassyType: embassy_rp::i2c::SclPin<Self>> + 'static,
    >(
        world: &mut World,
        config: embassy_rp::i2c::Config,
    ) -> Result<embassy_rp::i2c::I2c<'static, Self, embassy_rp::i2c::Blocking>, MakeI2CError> {
        let Some(pac) = world.remove_non_send_resource::<Peri<'static, Self>>() else {
            #[cfg(feature = "defmt")]
            defmt::error!("{} peripheral has already been taken", Self::NAME);
            return Err(MakeI2CError::PeripheralTaken);
        };
        let Some(sda) = SDA::from_world(world) else {
            #[cfg(feature = "defmt")]
            defmt::error!(
                "Tx({}) pin for {} has already been taken",
                SDA::NAME,
                Self::NAME
            );
            // if tx pin is taken, put back pac
            world.insert_non_send_resource(pac);
            return Err(MakeI2CError::SDATaken);
        };
        let Some(scl) = SCL::from_world(world) else {
            #[cfg(feature = "defmt")]
            defmt::error!(
                "Rx({}) pin for {} has already been taken",
                SCL::NAME,
                Self::NAME
            );
            // if rx pin is taken, put back pac and tx
            world.insert_non_send_resource(pac);
            world.insert_non_send_resource(sda);
            return Err(MakeI2CError::SCLTaken);
        };
        Ok(embassy_rp::i2c::I2c::new_blocking(pac, scl, sda, config))
    }
}

pub mod i2c0 {
    use super::*;
    use embassy_rp::peripherals::I2C0;
    use pico_bevy_core::gpio::*;
    impl I2CPlugin<I2C0> {
        pub fn i2c0(sda: SDAPins, scl: SCLPins) -> Self {
            I2CPlugin {
                sda,
                scl,
                config: embassy_rp::i2c::Config::default(),
            }
        }
    }

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy, Default)]
    pub enum SDAPins {
        Gpio0 = 0,
        #[default]
        Gpio4 = 4,
        Gpio8 = 8,
        Gpio12 = 12,
        Gpio16 = 16,
        Gpio20 = 20,
    }

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy, Default)]
    pub enum SCLPins {
        Gpio1 = 0,
        #[default]
        Gpio5 = 4,
        Gpio9 = 8,
        Gpio13 = 12,
        Gpio17 = 16,
        Gpio21 = 20,
    }

    impl I2CPeripheral for I2C0 {
        type SDAPins = SDAPins;
        type SCLPins = SCLPins;
        const NAME: &'static str = "I2C0";
        fn get_i2c(
            world: &mut World,
            sda_pin: Self::SDAPins,
            scl_pin: Self::SCLPins,
            config: embassy_rp::i2c::Config,
        ) -> Result<embassy_rp::i2c::I2c<'static, I2C0, embassy_rp::i2c::Blocking>, MakeI2CError>
        {
            match (sda_pin, scl_pin) {
                (SDAPins::Gpio0, SCLPins::Gpio1) => Self::make_i2c::<GPIO0, GPIO1>(world, config),
                (SDAPins::Gpio0, SCLPins::Gpio5) => Self::make_i2c::<GPIO0, GPIO5>(world, config),
                (SDAPins::Gpio0, SCLPins::Gpio9) => Self::make_i2c::<GPIO0, GPIO9>(world, config),
                (SDAPins::Gpio0, SCLPins::Gpio13) => Self::make_i2c::<GPIO0, GPIO13>(world, config),
                (SDAPins::Gpio0, SCLPins::Gpio17) => Self::make_i2c::<GPIO0, GPIO17>(world, config),
                (SDAPins::Gpio0, SCLPins::Gpio21) => Self::make_i2c::<GPIO0, GPIO21>(world, config),
                (SDAPins::Gpio4, SCLPins::Gpio1) => Self::make_i2c::<GPIO4, GPIO1>(world, config),
                (SDAPins::Gpio4, SCLPins::Gpio5) => Self::make_i2c::<GPIO4, GPIO5>(world, config),
                (SDAPins::Gpio4, SCLPins::Gpio9) => Self::make_i2c::<GPIO4, GPIO9>(world, config),
                (SDAPins::Gpio4, SCLPins::Gpio13) => Self::make_i2c::<GPIO4, GPIO13>(world, config),
                (SDAPins::Gpio4, SCLPins::Gpio17) => Self::make_i2c::<GPIO4, GPIO17>(world, config),
                (SDAPins::Gpio4, SCLPins::Gpio21) => Self::make_i2c::<GPIO4, GPIO21>(world, config),
                (SDAPins::Gpio8, SCLPins::Gpio1) => Self::make_i2c::<GPIO8, GPIO1>(world, config),
                (SDAPins::Gpio8, SCLPins::Gpio5) => Self::make_i2c::<GPIO8, GPIO5>(world, config),
                (SDAPins::Gpio8, SCLPins::Gpio9) => Self::make_i2c::<GPIO8, GPIO9>(world, config),
                (SDAPins::Gpio8, SCLPins::Gpio13) => Self::make_i2c::<GPIO8, GPIO13>(world, config),
                (SDAPins::Gpio8, SCLPins::Gpio17) => Self::make_i2c::<GPIO8, GPIO17>(world, config),
                (SDAPins::Gpio8, SCLPins::Gpio21) => Self::make_i2c::<GPIO8, GPIO21>(world, config),
                (SDAPins::Gpio12, SCLPins::Gpio1) => Self::make_i2c::<GPIO12, GPIO1>(world, config),
                (SDAPins::Gpio12, SCLPins::Gpio5) => Self::make_i2c::<GPIO12, GPIO5>(world, config),
                (SDAPins::Gpio12, SCLPins::Gpio9) => Self::make_i2c::<GPIO12, GPIO9>(world, config),
                (SDAPins::Gpio12, SCLPins::Gpio13) => {
                    Self::make_i2c::<GPIO12, GPIO13>(world, config)
                }
                (SDAPins::Gpio12, SCLPins::Gpio17) => {
                    Self::make_i2c::<GPIO12, GPIO17>(world, config)
                }
                (SDAPins::Gpio12, SCLPins::Gpio21) => {
                    Self::make_i2c::<GPIO12, GPIO21>(world, config)
                }
                (SDAPins::Gpio16, SCLPins::Gpio1) => Self::make_i2c::<GPIO16, GPIO1>(world, config),
                (SDAPins::Gpio16, SCLPins::Gpio5) => Self::make_i2c::<GPIO16, GPIO5>(world, config),
                (SDAPins::Gpio16, SCLPins::Gpio9) => Self::make_i2c::<GPIO16, GPIO9>(world, config),
                (SDAPins::Gpio16, SCLPins::Gpio13) => {
                    Self::make_i2c::<GPIO16, GPIO13>(world, config)
                }
                (SDAPins::Gpio16, SCLPins::Gpio17) => {
                    Self::make_i2c::<GPIO16, GPIO17>(world, config)
                }
                (SDAPins::Gpio16, SCLPins::Gpio21) => {
                    Self::make_i2c::<GPIO16, GPIO21>(world, config)
                }
                (SDAPins::Gpio20, SCLPins::Gpio1) => Self::make_i2c::<GPIO20, GPIO1>(world, config),
                (SDAPins::Gpio20, SCLPins::Gpio5) => Self::make_i2c::<GPIO20, GPIO5>(world, config),
                (SDAPins::Gpio20, SCLPins::Gpio9) => Self::make_i2c::<GPIO20, GPIO9>(world, config),
                (SDAPins::Gpio20, SCLPins::Gpio13) => {
                    Self::make_i2c::<GPIO20, GPIO13>(world, config)
                }
                (SDAPins::Gpio20, SCLPins::Gpio17) => {
                    Self::make_i2c::<GPIO20, GPIO17>(world, config)
                }
                (SDAPins::Gpio20, SCLPins::Gpio21) => {
                    Self::make_i2c::<GPIO20, GPIO21>(world, config)
                }
            }
        }
    }
}

pub mod i2c1 {
    use embassy_rp::peripherals::*;
    use pico_bevy_core::gpio::*;

    use super::*;
    impl I2CPlugin<I2C1> {
        pub fn i2c1(sda: SDAPins, scl: SCLPins) -> Self {
            I2CPlugin {
                sda,
                scl,
                config: embassy_rp::i2c::Config::default(),
            }
        }
    }
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy)]
    pub enum SDAPins {
        Gpio2 = 2,
        Gpio6 = 6,
        Gpio10 = 10,
        Gpio14 = 14,
        Gpio18 = 18,
        Gpio26 = 26,
    }
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy)]
    pub enum SCLPins {
        Gpio3 = 3,
        Gpio7 = 7,
        Gpio11 = 11,
        Gpio15 = 15,
        Gpio19 = 19,
        Gpio27 = 27,
    }
    impl I2CPeripheral for I2C1 {
        type SDAPins = SDAPins;
        type SCLPins = SCLPins;
        const NAME: &'static str = "I2C1";
        fn get_i2c(
            world: &mut World,
            sda_pin: Self::SDAPins,
            scl_pin: Self::SCLPins,
            config: embassy_rp::i2c::Config,
        ) -> Result<embassy_rp::i2c::I2c<'static, I2C1, embassy_rp::i2c::Blocking>, MakeI2CError>
        {
            match (sda_pin, scl_pin) {
                (SDAPins::Gpio2, SCLPins::Gpio3) => Self::make_i2c::<GPIO2, GPIO3>(world, config),
                (SDAPins::Gpio2, SCLPins::Gpio7) => Self::make_i2c::<GPIO2, GPIO7>(world, config),
                (SDAPins::Gpio2, SCLPins::Gpio11) => Self::make_i2c::<GPIO2, GPIO11>(world, config),
                (SDAPins::Gpio2, SCLPins::Gpio15) => Self::make_i2c::<GPIO2, GPIO15>(world, config),
                (SDAPins::Gpio2, SCLPins::Gpio19) => Self::make_i2c::<GPIO2, GPIO19>(world, config),
                (SDAPins::Gpio2, SCLPins::Gpio27) => Self::make_i2c::<GPIO2, GPIO27>(world, config),
                (SDAPins::Gpio6, SCLPins::Gpio3) => Self::make_i2c::<GPIO6, GPIO3>(world, config),
                (SDAPins::Gpio6, SCLPins::Gpio7) => Self::make_i2c::<GPIO6, GPIO7>(world, config),
                (SDAPins::Gpio6, SCLPins::Gpio11) => Self::make_i2c::<GPIO6, GPIO11>(world, config),
                (SDAPins::Gpio6, SCLPins::Gpio15) => Self::make_i2c::<GPIO6, GPIO15>(world, config),
                (SDAPins::Gpio6, SCLPins::Gpio19) => Self::make_i2c::<GPIO6, GPIO19>(world, config),
                (SDAPins::Gpio6, SCLPins::Gpio27) => Self::make_i2c::<GPIO6, GPIO27>(world, config),
                (SDAPins::Gpio10, SCLPins::Gpio3) => Self::make_i2c::<GPIO10, GPIO3>(world, config),
                (SDAPins::Gpio10, SCLPins::Gpio7) => Self::make_i2c::<GPIO10, GPIO7>(world, config),
                (SDAPins::Gpio10, SCLPins::Gpio11) => {
                    Self::make_i2c::<GPIO10, GPIO11>(world, config)
                }
                (SDAPins::Gpio10, SCLPins::Gpio15) => {
                    Self::make_i2c::<GPIO10, GPIO15>(world, config)
                }
                (SDAPins::Gpio10, SCLPins::Gpio19) => {
                    Self::make_i2c::<GPIO10, GPIO19>(world, config)
                }
                (SDAPins::Gpio10, SCLPins::Gpio27) => {
                    Self::make_i2c::<GPIO10, GPIO27>(world, config)
                }
                (SDAPins::Gpio14, SCLPins::Gpio3) => Self::make_i2c::<GPIO14, GPIO3>(world, config),
                (SDAPins::Gpio14, SCLPins::Gpio7) => Self::make_i2c::<GPIO14, GPIO7>(world, config),
                (SDAPins::Gpio14, SCLPins::Gpio11) => {
                    Self::make_i2c::<GPIO14, GPIO11>(world, config)
                }
                (SDAPins::Gpio14, SCLPins::Gpio15) => {
                    Self::make_i2c::<GPIO14, GPIO15>(world, config)
                }
                (SDAPins::Gpio14, SCLPins::Gpio19) => {
                    Self::make_i2c::<GPIO14, GPIO19>(world, config)
                }
                (SDAPins::Gpio14, SCLPins::Gpio27) => {
                    Self::make_i2c::<GPIO14, GPIO27>(world, config)
                }
                (SDAPins::Gpio18, SCLPins::Gpio3) => Self::make_i2c::<GPIO18, GPIO3>(world, config),
                (SDAPins::Gpio18, SCLPins::Gpio7) => Self::make_i2c::<GPIO18, GPIO7>(world, config),
                (SDAPins::Gpio18, SCLPins::Gpio11) => {
                    Self::make_i2c::<GPIO18, GPIO11>(world, config)
                }
                (SDAPins::Gpio18, SCLPins::Gpio15) => {
                    Self::make_i2c::<GPIO18, GPIO15>(world, config)
                }
                (SDAPins::Gpio18, SCLPins::Gpio19) => {
                    Self::make_i2c::<GPIO18, GPIO19>(world, config)
                }
                (SDAPins::Gpio18, SCLPins::Gpio27) => {
                    Self::make_i2c::<GPIO18, GPIO27>(world, config)
                }
                (SDAPins::Gpio26, SCLPins::Gpio3) => Self::make_i2c::<GPIO26, GPIO3>(world, config),
                (SDAPins::Gpio26, SCLPins::Gpio7) => Self::make_i2c::<GPIO26, GPIO7>(world, config),
                (SDAPins::Gpio26, SCLPins::Gpio11) => {
                    Self::make_i2c::<GPIO26, GPIO11>(world, config)
                }
                (SDAPins::Gpio26, SCLPins::Gpio15) => {
                    Self::make_i2c::<GPIO26, GPIO15>(world, config)
                }
                (SDAPins::Gpio26, SCLPins::Gpio19) => {
                    Self::make_i2c::<GPIO26, GPIO19>(world, config)
                }
                (SDAPins::Gpio26, SCLPins::Gpio27) => {
                    Self::make_i2c::<GPIO26, GPIO27>(world, config)
                }
            }
        }
    }
}
