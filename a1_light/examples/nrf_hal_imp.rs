#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let _pin_p0_19 = gpio_p0.p0_19.into_push_pull_output(Level::High);
    let _pin_p0_30 = gpio_p0.p0_30.into_push_pull_output(Level::Low);

    loop {}
}
