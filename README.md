Pico Bevy is a collection of crates to make it easy to make a Bevy app on the RP Pico

# Quick Start
```rust
use bevy::prelude::*;
use pico_bevy::*;
#[cortex_m_rt::entry]
fn main() -> ! {
    //this inits the heap; can be disabled with no_default_features; default heap is 100kb
    pico_bevy::init();
    let mut app = App::new();
    // can config the clocks by inserting embassy_rp::clocks::ClockConfig as non_send_resource
    app.add_plugins(PicoCore); // calls embassy_rp::init() & inserts peripherals;
    app.add_plugins(PicoUArtPlugin::default()); // can use ::uart0(tx, rx) or ::uart1(tx, rx) to set custom pins; default uses UART0, PIN_0, PIN_1
    app.set_runner(app_runner); // you have to do this yourself for now; the runner I'm using for testing requires me to finish the Pico-Bevy-Time crate
    app.run();
}

fn app_runner(mut app: App) -> AppExit {
    // run startup
    let plugins_state = app.plugins_state();
    if plugins_state != bevy::app::PluginsState::Cleaned {
        while app.plugins_state() == bevy::app::PluginsState::Adding {
            bevy::tasks::tick_global_task_pools_on_main_thread();
        }
        app.finish();
        app.cleanup();
    }
    //enable timer 1 interrupts
    unsafe {
        rp_pac::Interrupt::TIMER_IRQ_1.enable();
    }
    let frame_stride = 1_000_000; // time in us
    //enable timer 1
    rp_pac::TIMER.inte().write(|w| w.set_alarm(1, true));
    let mut next = pico::time::now(); // just return TIMER time u64
    loop {
        next += frame_stride;
        app.update();
        //don't sleep if it took longer than one frame to run
        if pico::time::now() > next {
            next = pico::time::new(); // set next to now, so if it's taking too long, we don't do lots of fast frames to catch up
            continue;
        }
        rp_pac::TIMER.alarm(1).write_value(next as u32); set alarm for start of next frame
        cortex_m::asm::wfi(); // sleep the core till the next frame should start <- should probably make this a loop so random intrupts dont break frame timing
    }
}
```

Currently, I have made 2 crates
# Pico-Bevy-Core
this is the core crate<br>
Its job is to configure the app for the other plugins to then use<br>
For now, I am using embassy_rp as my rp2040 interface<br>
Adding the PicoCore plugin to your app will call `embassy_rp::init()` and add the peripherals to the app as none_send_resources<br>
Pico-Bevy-Core uses features to determine what peripherals are added to the world; this cuts down on the amount of memory used by not adding unneeded peripherals.<br>
## currently: 
- uart
- gpio
- spi
- i2c
- watchdog
- rtc

# Pico-Bevy-Uart
This crate adds UART functionality<br>
By adding the `PicoUArtPlugin<UART*>` to your app, you get access to a `UArtBus<UART*>` resource that you can use in systems.<br>
Each UART* has a custom impl that means you can only configure valid pins.
