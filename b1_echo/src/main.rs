#![no_main]
#![no_std]

use cortex_m_rt::entry;
use core::fmt::Write;
use heapless::Vec;
use microbit::{hal::{uarte::{Baudrate, Parity}, Uarte}, Board};
use panic_rtt_target as _;
use serial_utils::UartePort;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut serial = {
        let serial = Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200
        );
        UartePort::new(serial)
    };
    let mut buffer: Vec<char, 32> = Vec::new();

    loop {
        buffer.clear();
        let mut input: u8 = serial.read().unwrap();
        
        while input != 13 {
            let input_char = input as char;
            serial.write(input_char as u8).unwrap();
            if buffer.push(input_char).is_err() {
                write!(serial, "\r\nerror: buffer full").unwrap();
                break;
            }
            input = serial.read().unwrap();
        }

        rprintln!("{:?}", buffer);
        write!(serial, "\r\n").unwrap();
        for byte in buffer.iter(){
            write!(serial, "{}", *byte).unwrap();
        }
        write!(serial, "\r\n").unwrap();
        
        serial.flush().unwrap();
    }
}
