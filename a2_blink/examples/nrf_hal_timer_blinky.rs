#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::StatefulOutputPin;
use microbit::Peripherals;
use nrf52833_hal::{
    gpio::{p0, Level},
    timer,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let _pin_p0_19 = gpio_p0.p0_19.into_push_pull_output(Level::High);
    let mut pin_p0_30 = gpio_p0.p0_30.into_push_pull_output(Level::Low);
    let mut timer_0 = timer::Timer::new(peripherals.TIMER0);

    let start = timer_0.read();
    timer_0.start(1_000_000u32 * 10);
    // 1_000_000 Hz = 1 MHz (freequency of timer).
    // So 1_000_000 cycles make a second.
    // 1_000_000u32 * 10 means the timer should run for 10 seconds.
    let compare_time: u32 = 1_000_000u32;

    loop {
        if timer_0.read() - start >= compare_time {
            pin_p0_30.toggle().unwrap();
            timer_0.start(1_000_000u32 * 10);
            // restarting the timer, so it keeps running indefinitely
        }
    }
}
