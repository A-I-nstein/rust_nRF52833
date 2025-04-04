#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
    pwm::{Channel, Pwm},
    time::U32Ext,
};
use panic_halt as _;

fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().expect("Couldn't find the board.");
    let p0 = p0::Parts::new(peripherals.P0);
    let pwm = Pwm::new(peripherals.PWM0);

    let _row_5 = p0.p0_19.into_push_pull_output(Level::High);

    pwm.set_output_pin(
        Channel::C0,
        p0.p0_30.into_push_pull_output(Level::High).degrade(),
    );
    pwm.set_period(500_u32.hz());

    let large_value = 100_000u32;

    loop {
        for duty in 0..large_value {
            pwm.set_duty_on_common(
                map(duty.into(), 0, large_value, 0, pwm.max_duty().into())
                    .try_into()
                    .unwrap(),
            );
        }
        for duty in (0..large_value).rev() {
            pwm.set_duty_on_common(
                map(duty.into(), 0, large_value, 0, pwm.max_duty().into())
                    .try_into()
                    .unwrap(),
            );
        }
    }
}
