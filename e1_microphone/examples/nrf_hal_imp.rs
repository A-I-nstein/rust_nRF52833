#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use nrf52833_hal::{
    gpio::{p0, Level, OpenDrainConfig}, pac::Peripherals, saadc::SaadcConfig, timer, Saadc
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let peripherals = Peripherals::take().unwrap();
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let mut mic_in = gpio_p0.p0_05.into_floating_input();
    let _mic_run = gpio_p0
        .p0_20
        .into_open_drain_output(OpenDrainConfig::Disconnect0HighDrive1, Level::High);
    let saadc_config = SaadcConfig::default();
    let mut saadc = Saadc::new(peripherals.SAADC, saadc_config);
    let mut timer_0 = timer::Timer::new(peripherals.TIMER0);

    loop {
        timer_0.delay_ms(500u32);
        let mic_reading = saadc
            .read_channel(&mut mic_in)
            .expect("Could not read microphone value") as u16;
        rprintln!("Mic Reading: {}", mic_reading);
    }
}
