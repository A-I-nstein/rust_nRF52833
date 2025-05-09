#![no_std]
#![no_main]

use core::fmt::Write;
use hal::{gpio, uarte, uarte::Uarte};
use nrf52833_hal as hal;
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();

    let (uart0, cdc_pins) = {
        let p0 = gpio::p0::Parts::new(p.P0);
        (
            p.UARTE0,
            uarte::Pins {
                txd: p0.p0_06.into_push_pull_output(gpio::Level::High).degrade(),
                rxd: p0.p0_08.into_floating_input().degrade(),
                cts: None,
                rts: None,
            },
        )
    };

    let mut uarte = Uarte::new(
        uart0,
        cdc_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    loop {
        write!(uarte, "Hello, World!\r\n").unwrap();
    }
}
