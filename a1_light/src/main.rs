#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::Board;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.row5.set_high().unwrap();
    board.display_pins.col5.set_low().unwrap();

    loop {}
}
