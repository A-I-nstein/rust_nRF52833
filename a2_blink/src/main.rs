#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{
    hal::{gpio::Level, Timer},
    Board,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut row5 = board.display_pins.row5.into_push_pull_output(Level::High);
    let _col5 = board.display_pins.col5.into_push_pull_output(Level::Low);
    let mut timer0 = Timer::new(board.TIMER0);

    loop {
        timer0.delay_ms(500);
        row5.set_low().unwrap();
        timer0.delay_ms(500);
        row5.set_high().unwrap();
    }
}
