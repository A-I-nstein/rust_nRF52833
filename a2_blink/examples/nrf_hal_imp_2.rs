#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
    timer,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let mut timer_0 = timer::Timer::new(peripherals.TIMER0);
    let _pin_p0_31 = gpio_p0.p0_31.into_push_pull_output(Level::Low);
    let mut pin_1 = gpio_p0.p0_21.into_push_pull_output(Level::High);
    let mut pin_2 = gpio_p0.p0_22.into_push_pull_output(Level::High);
    let mut pin_3 = gpio_p0.p0_15.into_push_pull_output(Level::High);
    let mut pin_4 = gpio_p0.p0_24.into_push_pull_output(Level::High);
    let mut pin_5 = gpio_p0.p0_19.into_push_pull_output(Level::High);

    loop {
        timer_0.delay_ms(1_000u32);

        pin_1.set_low().unwrap();
        pin_2.set_low().unwrap();
        pin_3.set_low().unwrap();
        pin_4.set_low().unwrap();
        pin_5.set_low().unwrap();
        timer_0.delay_ms(1_000u32);

        pin_1.set_high().unwrap();
        timer_0.delay_ms(1_000u32);
        pin_2.set_high().unwrap();
        timer_0.delay_ms(1_000u32);
        pin_3.set_high().unwrap();
        timer_0.delay_ms(1_000u32);
        pin_4.set_high().unwrap();
        timer_0.delay_ms(1_000u32);
        pin_5.set_high().unwrap();
    }
}
