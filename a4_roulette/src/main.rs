#![no_main]
#![no_std]

use a4_utils::{Direction, DisplayData};
use cortex_m_rt::entry;
use microbit::{display::blocking::Display, hal::Timer, Board};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut display_data = DisplayData {
        curr_pos: (0, 0),
        direction: Direction::Right,
    };

    let mut display_state: [[u8; 5]; 5] =
        [[0; DisplayData::GRID_SIZE as usize]; DisplayData::GRID_SIZE as usize];
    display_state[display_data.curr_pos.0 as usize][display_data.curr_pos.1 as usize] = 1;

    loop {
        display_state[display_data.curr_pos.0 as usize][display_data.curr_pos.1 as usize] = 0;
        display_data.get_next_pos();
        display_state[display_data.curr_pos.0 as usize][display_data.curr_pos.1 as usize] = 1;
        display.show(&mut timer, display_state, 100);
        display.clear();
    }
}
