#![no_main]
#![no_std]

use cortex_m_rt::entry;
use core::fmt::Write;
use core::str::from_utf8;
use heapless::Vec;
use lsm303agr::{
    AccelMode, AccelOutputDataRate, 
    Lsm303agr, 
    MagMode, MagOutputDataRate
};
use microbit::{
    hal::{uarte::{Baudrate, Parity}, Timer, Twim, Uarte}, 
    pac::twim0::frequency::FREQUENCY_A, Board
};
use panic_halt as _;
use serial_utils::UartePort;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer0 = Timer::new(board.TIMER0);
    let i2c = Twim::new(
        board.TWIM0, 
        board.i2c_internal.into(), 
        FREQUENCY_A::K100
    );
    let mut serial = {
        let serial = Uarte::new(
            board.UARTE0, 
            board.uart.into(), 
            Parity::EXCLUDED, 
            Baudrate::BAUD115200
        );
        UartePort::new(serial)
    };
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    let mut buffer:Vec<u8, 32> = Vec::new();

    sensor.init().unwrap();
    sensor.set_accel_mode_and_odr(&mut timer0, AccelMode::HighResolution, AccelOutputDataRate::Hz10).unwrap();
    sensor.set_mag_mode_and_odr(&mut timer0, MagMode::HighResolution, MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        buffer.clear();
        let mut input: u8 = serial.read().unwrap();  
        while input != 13 {
            serial.write(input).unwrap();
            if buffer.push(input).is_err() {
                break;
            };
            input = serial.read().unwrap();
        }

        let peripheral = from_utf8(&buffer).unwrap();
        match peripheral {
            "accelerometer" => {
                if sensor.accel_status().unwrap().xyz_new_data() {
                    let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
                    write!(serial, "\r\nAcceleration: x {} y {} z {}\r\n", x, y, z).unwrap();
                }
            },
            "magnetometer" => {
                if sensor.mag_status().unwrap().xyz_new_data() {
                    let (x, y, z) = sensor.magnetic_field().unwrap().xyz_nt();
                    write!(serial, "\r\nMagnetic Field: x {} y {} z {}\r\n", x, y, z).unwrap();
                }
            },
            _ => {
                write!(serial, "\r\nInvalid command.\r\n").unwrap();
            }
        }
    }
}