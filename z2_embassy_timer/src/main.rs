#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_init_print!();
    let mut time = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    let _peripherals = embassy_nrf::init(Default::default());

    loop {
        Timer::after_millis(1000).await;
        time.seconds = time.seconds.wrapping_add(1);

        if time.seconds > 59 {
            time.seconds = 0;
            time.minutes += 1;
        }
        if time.minutes > 59 {
            time.minutes = 0;
            time.hours += 1;
        }
        if time.hours > 23 {
            time.seconds = 0;
            time.minutes = 0;
            time.hours = 0;
        }
        rprintln!(
            "Elapsed Time {:0>2}:{:0>2}:{:0>2}",
            time.hours,
            time.minutes,
            time.seconds
        )
    }
}
