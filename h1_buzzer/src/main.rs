#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{
    gpio::{p0, Level},
    pac::Peripherals,
    pwm::{Channel, Pwm},
    time::U32Ext,
    Timer,
};
use panic_halt as _;

// Musical Notes and their frequency in Hz
// C    C#  D   D#  E   F   F#  G   G#  A   A#  B
// 262  277 294 311 330 349 370 392 415 440 466 494

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().expect("Couldn't find the board.");
    let p0 = p0::Parts::new(peripherals.P0);
    let mut timer = Timer::new(peripherals.TIMER0);
    let mut speaker_pin = p0.p0_00.into_push_pull_output(Level::High);
    let _ = speaker_pin.set_low();

    let speaker = Pwm::new(peripherals.PWM0);
    speaker.set_output_pin(Channel::C0, speaker_pin.degrade());

    let happy_birthday = [
        262, 262, 294, 262, 349, 330, 262, 262, 294, 262, 392, 349, 262, 262, 523, 440, 349, 330,
        294, 466, 466, 440, 349, 392, 349,
    ];

    loop {
        for frequency in happy_birthday {
            speaker.set_period((frequency * 10).hz()); // multiplying by 10 plays the music
            speaker.set_duty_on_common(speaker.max_duty() / 2);
            timer.delay_ms(200);
            speaker.set_duty_on_common(0);
            timer.delay_ms(50);
        }
        timer.delay_ms(1000);
    }
}
