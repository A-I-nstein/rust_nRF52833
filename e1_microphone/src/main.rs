#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::
    {
        display::blocking::Display, 
        hal::{
            gpio::{
                Level, 
                OpenDrainConfig
            }, 
            saadc::SaadcConfig, 
            Saadc, 
            Timer
        }, 
        Board
    };
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board
        ::take()
        .expect("Could not find the board");
    let mut timer0 = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let saadc_config = SaadcConfig::default();
    let mut saadc = Saadc::new(
        board.ADC, 
        saadc_config
    );
    let mut mic_in = board
        .microphone_pins
        .mic_in
        .into_floating_input();

    board
        .microphone_pins
        .mic_run
        .into_open_drain_input_output(
            OpenDrainConfig::Disconnect0HighDrive1, 
            Level::High
        );

    let mut count: u64 = 0;
    let mut sum: u64 = 0;
    let mut max_value: u16 = 0;

    loop {
        let mic_value = saadc
            .read_channel(&mut mic_in)
            .expect("Could not read microphone value") as u16;

        max_value = max_value.max(mic_value);
        sum += mic_value as u64;
        count += 1;

        if count %100 == 0 {
            let avg = (sum / count) as u16;
            let image = [
                [if max_value > avg + 100 {1} else {0}; 5],
                [if max_value > avg + 80 {1} else {0}; 5],
                [if max_value > avg + 60 {1} else {0}; 5],
                [if max_value > avg + 40 {1} else {0}; 5],
                [if max_value > avg + 20 {1} else {0}; 5],
            ];
            display.show(&mut timer0, image, 10);
            max_value = 0;
        }
    }
}
