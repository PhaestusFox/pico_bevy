Pico Bevy is a collection of crates to make it easy to make a Bevy app on the RP Pico

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
