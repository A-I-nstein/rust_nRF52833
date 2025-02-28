#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{display::blocking::Display, hal::Timer, Board};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let display_state = [[1; 5]; 5];

    loop {
        display.show(&mut timer, display_state, 1000);
        display.clear();
        timer.delay_ms(1000);
    }
}
