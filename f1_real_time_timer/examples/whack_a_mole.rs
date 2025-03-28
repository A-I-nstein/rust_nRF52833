#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, StatefulOutputPin};
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
    timer::Instance,
    Rng,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = Peripherals::take().expect("Could not find board.");
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let _row_1 = gpio_p0.p0_21.into_push_pull_output(Level::High);
    let mut col_5 = gpio_p0.p0_30.into_push_pull_output(Level::High);
    let mut button_b = gpio_p0.p0_23.into_pullup_input();
    let mut range = Rng::new(peripherals.RNG);
    let timer_0 = peripherals.TIMER0;
    let timer_1 = peripherals.TIMER1;
    let mut best_response_time = u32::MAX;

    timer_0.set_periodic();
    timer_1.set_periodic();
    rprintln!("Game begins! Press button B when the led glows.");

    loop {
        let mut mole_whacked = false;
        let mut response_time = 0_u32;
        let mut init_delay_s = (range.random_u8() / 51_u8) as u32;
        if init_delay_s == 0 {
            init_delay_s = 1;
        }
        timer_0.timer_start(1_000_000_u32 * init_delay_s);
        while timer_0.timer_running() {}
        col_5.toggle().expect("Could not toggle led.");
        let mut blink_delay_ms = (range.random_u8() * 6_u8) as u32;
        while blink_delay_ms < 500_u32 {
            blink_delay_ms *= 2_u32;
        }
        timer_0.timer_start(1_000_000_u32 * blink_delay_ms / 1_000_u32);
        timer_1.timer_start(1_000_000_u32 * 10_u32);
        while timer_0.timer_running() {
            if button_b.is_low().expect("Could not read button state") {
                response_time = timer_1.read_counter() / 1_000_u32;
                if response_time < best_response_time {
                    best_response_time = response_time;
                }
                timer_1.timer_cancel();
                mole_whacked = true;
            }
        }
        col_5.toggle().expect("Could not toggle led.");
        if !mole_whacked {
            rprintln!("The mole escaped");
        } else {
            rprintln!("You have whacked the mole! Response Time: {}, Best Response Time: {}", response_time, best_response_time);
        }
    }
}
