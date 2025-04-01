#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
    pwm::{Channel, Pwm},
    time::U32Ext,
    Timer,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().expect("Couldn't find the board.");
    let p0 = p0::Parts::new(peripherals.P0);
    let pwm = Pwm::new(peripherals.PWM0);
    let mut timer = Timer::new(peripherals.TIMER0);
    
    let _row_5 = p0.p0_19.into_push_pull_output(Level::High);
    let wait_time = 1_000_000u32 / pwm.max_duty() as u32;

    pwm.set_output_pin(
        Channel::C0,
        p0.p0_30.into_push_pull_output(Level::High).degrade(),
    );
    pwm.set_period(500_u32.hz());
    
    loop {
        for duty in 0..pwm.max_duty() {
            pwm.set_duty_on_common(duty);
            timer.delay(wait_time);
        }
        for duty in (0..pwm.max_duty()).rev() {
            pwm.set_duty_on_common(duty);
            timer.delay(wait_time);
        }
    }

}
