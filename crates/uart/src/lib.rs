#![no_std]
mod plugin;
use bevy::{
    ecs::{resource::Resource, world::World},
    prelude::{Deref, DerefMut},
};
use embassy_rp::{
    Peri,
    peripherals::UART0,
    uart::{RxPin, TxPin},
};
use pico_bevy_core::{UseBus, gpio::PicoPin};
pub use plugin::{MakeUArtError, UArtPlugin};

pub type UArtError = embassy_rp::uart::Error;

#[derive(Resource, Deref, DerefMut)]
pub struct UArtBus<P: UArtPeripheral> {
    #[deref]
    bus: embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>,
    peripheral: core::marker::PhantomData<P>,
}

impl<P: UArtPeripheral> UArtBus<P> {
    pub fn new(bus: embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>) -> Self {
        UArtBus {
            bus,
            peripheral: core::marker::PhantomData,
        }
    }
}

pub trait UArtPeripheral: embassy_rp::uart::Instance + 'static {
    #[cfg(feature = "defmt")]
    type TxPins: Send + Sync + Copy + 'static + defmt::Format;
    #[cfg(not(feature = "defmt"))]
    type TxPins: Send + Sync + Copy + 'static;
    #[cfg(feature = "defmt")]
    type RxPins: Send + Sync + Copy + 'static + defmt::Format;
    #[cfg(not(feature = "defmt"))]
    type RxPins: Send + Sync + Copy + 'static;
    const NAME: &'static str;
    fn get_uart(
        world: &mut World,
        tx_pin: Self::TxPins,
        rx_pin: Self::RxPins,
        config: embassy_rp::uart::Config,
    ) -> Result<embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>, MakeUArtError>;
    fn get_pin<T: PicoPin>(world: &mut World) -> Option<Peri<'static, T::EmbassyType>> {
        world.remove_non_send_resource::<Peri<'static, T::EmbassyType>>()
    }
    fn make_uart<
        TX: PicoPin<EmbassyType: TxPin<Self>> + 'static,
        RX: PicoPin<EmbassyType: RxPin<Self>> + 'static,
    >(
        world: &mut World,
        config: embassy_rp::uart::Config,
    ) -> Result<embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>, MakeUArtError> {
        let Some(pac) = world.remove_non_send_resource::<Peri<'static, Self>>() else {
            #[cfg(feature = "defmt")]
            defmt::error!("{} peripheral has already been taken", Self::NAME);
            return Err(MakeUArtError::PeripheralTaken);
        };
        let Some(tx) = TX::from_world(world) else {
            #[cfg(feature = "defmt")]
            defmt::error!(
                "Tx({}) pin for {} has already been taken",
                TX::NAME,
                Self::NAME
            );
            // if tx pin is taken, put back pac
            world.insert_non_send_resource(pac);
            return Err(MakeUArtError::TxTaken);
        };
        let Some(rx) = RX::from_world(world) else {
            #[cfg(feature = "defmt")]
            defmt::error!(
                "Rx({}) pin for {} has already been taken",
                RX::NAME,
                Self::NAME
            );
            // if rx pin is taken, put back pac and tx
            world.insert_non_send_resource(pac);
            world.insert_non_send_resource(tx);
            return Err(MakeUArtError::RxTaken);
        };
        Ok(embassy_rp::uart::Uart::new_blocking(pac, tx, rx, config))
    }
}

pub mod uart0 {
    use super::*;
    use pico_bevy_core::gpio::*;
    impl UArtPlugin<UART0> {
        pub fn uart0(tx: TxPins, rx: RxPins) -> Self {
            UArtPlugin {
                tx,
                rx,
                config: embassy_rp::uart::Config::default(),
            }
        }
    }

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy, Default)]
    pub enum TxPins {
        #[default]
        Gpio0 = 0,
        Gpio12 = 12,
        Gpio16 = 16,
    }

    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy, Default)]
    pub enum RxPins {
        #[default]
        Gpio1 = 1,
        Gpio13 = 13,
        Gpio17 = 17,
    }

    impl UArtPeripheral for UART0 {
        type TxPins = TxPins;
        type RxPins = RxPins;
        const NAME: &'static str = "UART0";
        fn get_uart(
            world: &mut World,
            tx_pin: Self::TxPins,
            rx_pin: Self::RxPins,
            config: embassy_rp::uart::Config,
        ) -> Result<embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>, MakeUArtError>
        {
            match (tx_pin, rx_pin) {
                (TxPins::Gpio0, RxPins::Gpio1) => Self::make_uart::<GPIO0, GPIO1>(world, config),
                (TxPins::Gpio12, RxPins::Gpio1) => Self::make_uart::<GPIO12, GPIO1>(world, config),
                (TxPins::Gpio16, RxPins::Gpio1) => Self::make_uart::<GPIO16, GPIO1>(world, config),
                (TxPins::Gpio0, RxPins::Gpio17) => Self::make_uart::<GPIO0, GPIO17>(world, config),
                (TxPins::Gpio16, RxPins::Gpio17) => {
                    Self::make_uart::<GPIO16, GPIO17>(world, config)
                }
                (TxPins::Gpio12, RxPins::Gpio17) => {
                    Self::make_uart::<GPIO12, GPIO17>(world, config)
                }
                (TxPins::Gpio12, RxPins::Gpio13) => {
                    Self::make_uart::<GPIO12, GPIO13>(world, config)
                }
                (TxPins::Gpio0, RxPins::Gpio13) => Self::make_uart::<GPIO0, GPIO13>(world, config),
                (TxPins::Gpio16, RxPins::Gpio13) => {
                    Self::make_uart::<GPIO16, GPIO13>(world, config)
                }
            }
        }
    }
}

pub mod uart1 {
    use embassy_rp::peripherals::*;
    use pico_bevy_core::gpio::*;

    use super::*;
    impl UArtPlugin<UART1> {
        pub fn uart1(tx: TxPins, rx: RxPins) -> Self {
            UArtPlugin {
                tx,
                rx,
                config: embassy_rp::uart::Config::default(),
            }
        }
    }
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy)]
    pub enum TxPins {
        Gpio4 = 4,
        Gpio8 = 8,
    }
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Clone, Copy)]
    pub enum RxPins {
        Gpio5 = 5,
        Gpio9 = 9,
    }
    impl UArtPeripheral for UART1 {
        type TxPins = TxPins;
        type RxPins = RxPins;
        const NAME: &'static str = "UART1";
        fn get_uart(
            world: &mut World,
            tx_pin: Self::TxPins,
            rx_pin: Self::RxPins,
            config: embassy_rp::uart::Config,
        ) -> Result<embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>, MakeUArtError>
        {
            match (tx_pin, rx_pin) {
                (TxPins::Gpio4, RxPins::Gpio5) => Self::make_uart::<GPIO4, GPIO5>(world, config),
                (TxPins::Gpio4, RxPins::Gpio9) => Self::make_uart::<GPIO4, GPIO9>(world, config),
                (TxPins::Gpio8, RxPins::Gpio5) => Self::make_uart::<GPIO8, GPIO5>(world, config),
                (TxPins::Gpio8, RxPins::Gpio9) => Self::make_uart::<GPIO8, GPIO9>(world, config),
            }
        }
    }
}

impl<P: UArtPeripheral> UArtBus<P> {
    pub fn write(&mut self, data: &[u8]) {
        // Blocking write can not error on pico
        _ = self.blocking_write(data);
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), embassy_rp::uart::Error> {
        self.blocking_read(buffer)
    }
}

pub trait UseUArtBus {
    fn uart0() -> pico_bevy_core::UseBus<embassy_rp::peripherals::UART0> {
        pico_bevy_core::UseBus::new()
    }
    fn uart1() -> pico_bevy_core::UseBus<embassy_rp::peripherals::UART1> {
        pico_bevy_core::UseBus::new()
    }
}

impl UseUArtBus for UseBus<()> {}
