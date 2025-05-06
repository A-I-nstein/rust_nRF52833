#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_nrf::init(Default::default());
    let _led_gnd = Output::new(peripherals.P0_19, Level::High, OutputDrive::Standard);
    let mut led = Output::new(peripherals.P0_30, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        Timer::after_millis(1_000).await;
        led.set_low();
        Timer::after_millis(1_000).await;
    }
}
