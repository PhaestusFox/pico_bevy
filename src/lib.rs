#![no_std]

#[cfg(feature = "heap")]
#[global_allocator]
pub static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();

pub mod prelude {
    pub use pico_bevy_core::*;
    pub use pico_bevy_uart::*;
}

pub use prelude::*;

#[cfg(feature = "heap_size_100kb")]
const HEAP_SIZE: usize = 100 * 1024; // 100 KB

pub fn init() {
    #[cfg(feature = "heap")]
    {
        static mut HEAP_MEM: [core::mem::MaybeUninit<u8>; HEAP_SIZE] =
            [core::mem::MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }
}
