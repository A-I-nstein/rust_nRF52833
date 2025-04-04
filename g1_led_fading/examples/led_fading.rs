#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
    Timer,
};
use panic_halt as _;

fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().expect("Couldn't find the board.");
    let p0 = p0::Parts::new(peripherals.P0);
    let mut timer = Timer::new(peripherals.TIMER0);

    let _row_5 = p0.p0_19.into_push_pull_output(Level::High);
    let mut led_pin = p0.p0_30.into_push_pull_output(Level::High);

    let frequency = 100;
    let period_ms = 2500 / frequency; // 25 ms period serves best

    loop {
        for intensity in 1..100 {
            let ontime_ms = map(intensity, 1, 100, 1, 25);
            let offtime_ms = period_ms - ontime_ms;
            led_pin.set_low().unwrap();
            timer.delay_ms(ontime_ms);
            led_pin.set_high().unwrap();
            timer.delay_ms(offtime_ms);
        }
        for intensity in (1..100).rev() {
            let ontime_ms = map(intensity, 1, 100, 1, 25);
            let offtime_ms = period_ms - ontime_ms;
            led_pin.set_low().unwrap();
            timer.delay_ms(ontime_ms);
            led_pin.set_high().unwrap();
            timer.delay_ms(offtime_ms);
        }
    }
}
