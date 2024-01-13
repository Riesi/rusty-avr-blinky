#![no_std]
#![no_main]
use atmega_hal::prelude::*;

use panic_halt as _;

#[avr_device::entry] // requires avr_device's rt feature.
fn main() -> ! {
    let mut delay = atmega_hal::delay::Delay::<atmega_hal::clock::MHz16>::new();
    let dp = atmega_hal::pac::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    let mut pb= pins.pb5.into_output();
    loop {
        pb.toggle();
        delay.delay_ms(500u16);
    }
}