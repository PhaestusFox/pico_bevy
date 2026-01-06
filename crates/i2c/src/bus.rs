use core::marker::PhantomData;

use bevy::ecs::component::Component;
use bevy::ecs::resource::Resource;
use embassy_rp::peripherals::{I2C0, I2C1};

use super::*;

#[derive(Resource, Deref, DerefMut)]
pub struct I2CBus<P: I2CPeripheral> {
    #[deref]
    bus: embassy_rp::i2c::I2c<'static, P, embassy_rp::i2c::Blocking>,
}

impl<P: I2CPeripheral> I2CBus<P> {
    pub fn new(bus: embassy_rp::i2c::I2c<'static, P, embassy_rp::i2c::Blocking>) -> Self {
        I2CBus { bus }
    }
}

#[derive(Component)]
pub struct UseBus<P: I2CPeripheral>(PhantomData<P>);

impl UseBus<I2C0> {
    pub fn i2c0() -> Self {
        UseBus(PhantomData)
    }
}

impl UseBus<I2C1> {
    pub fn i2c1() -> Self {
        UseBus(PhantomData)
    }
}

impl<P: I2CPeripheral> embedded_hal::i2c::ErrorType for I2CBus<P> {
    type Error = I2CError;
}

impl<P: I2CPeripheral> embedded_hal::i2c::I2c for I2CBus<P> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.bus.transaction(address, operations)
    }
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.blocking_read(address, read)
    }
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.bus.blocking_write(address, bytes)
    }
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.bus.blocking_write_read(address, bytes, buffer)
    }
}
