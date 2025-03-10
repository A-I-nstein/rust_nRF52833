#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, StatefulOutputPin};
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
};
use panic_rtt_target as _;
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let _pin_p0_19 = gpio_p0.p0_19.into_push_pull_output(Level::High);
    let mut pin_p0_30 = gpio_p0.p0_30.into_push_pull_output(Level::Low);
    let mut pin_p0_14 = gpio_p0.p0_14.into_pullup_input();

    let mut blnk_dly = 1_000_000_u32;

    loop {
        for _i in 0..blnk_dly {
            if pin_p0_14.is_low().unwrap() {
                blnk_dly -= 25_000_u32;
                if blnk_dly < 25_000_u32 {
                    blnk_dly = 1_000_000_u32;
                }
            }
        }
        pin_p0_30.toggle().unwrap();
    }
}
