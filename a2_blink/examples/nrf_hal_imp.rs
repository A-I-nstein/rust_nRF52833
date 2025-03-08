#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use nrf52833_hal::{gpio::{p0, Level}, pac::Peripherals, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let mut timer_0 = timer::Timer::new(peripherals.TIMER0);
    let _pin_p0_19 = gpio_p0.p0_19.into_push_pull_output(Level::High);
    let mut pin_p0_30 = gpio_p0.p0_30.into_push_pull_output(Level::Low);

    loop {
        timer_0.delay_ms(1_000u32);
        pin_p0_30.set_high().unwrap();
        timer_0.delay_ms(1_000u32);
        pin_p0_30.set_low().unwrap();
    }
}
