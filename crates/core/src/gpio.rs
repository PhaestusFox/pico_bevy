use bevy::ecs::world::World;
use embassy_rp::Peri;

pub trait PicoPin {
    const NAME: &'static str;
    type EmbassyType: embassy_rp::PeripheralType;
    fn from_world(world: &mut World) -> Option<Peri<'static, Self::EmbassyType>> {
        world.remove_non_send_resource::<Peri<'static, Self::EmbassyType>>()
    }
}

macro_rules! implPicoPin {
    // add a macro rule that matches multiple ',' separated literals
    ($first:literal, $($rest:literal),+) => {
        implPicoPin!($first);
        $(implPicoPin!($rest);)+
    };
    ($id:literal) => {
        impl PicoPin for paste::paste! { [<GPIO$id>] } {
            const NAME: &'static str = concat!("GPIO", stringify!($id));
            type EmbassyType = paste::paste! { embassy_rp::peripherals::[<PIN_$id>]};
        }
    };
}

implPicoPin!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29
);

pub struct GPIO0;
pub struct GPIO1;
pub struct GPIO2;
pub struct GPIO3;
pub struct GPIO4;
pub struct GPIO5;
pub struct GPIO6;
pub struct GPIO7;
pub struct GPIO8;
pub struct GPIO9;
pub struct GPIO10;
pub struct GPIO11;
pub struct GPIO12;
pub struct GPIO13;
pub struct GPIO14;
pub struct GPIO15;
pub struct GPIO16;
pub struct GPIO17;
pub struct GPIO18;
pub struct GPIO19;
pub struct GPIO20;
pub struct GPIO21;
pub struct GPIO22;
pub struct GPIO23;
pub struct GPIO24;
pub struct GPIO25;
pub struct GPIO26;
pub struct GPIO27;
pub struct GPIO28;
pub struct GPIO29;
