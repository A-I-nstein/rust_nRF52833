#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Timer;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

type ButtonType = Mutex<CriticalSectionRawMutex, Option<Input<'static>>>;

static BLINK_DELAY: AtomicU32 = AtomicU32::new(200_u32);
static BUTTON: ButtonType = Mutex::new(None);

#[embassy_executor::task]
async fn press_button(button: &'static ButtonType) {
    loop {
        {
            let mut button_unlocked = button.lock().await;
            if let Some(button_ref) = button_unlocked.as_mut() {
                button_ref.wait_for_falling_edge().await;
                rprintln!("Button Pressed!");
            }
        }
        let del = BLINK_DELAY.load(Ordering::Relaxed);
        if del <= 50_u32 {
            BLINK_DELAY.store(200_u32, Ordering::Relaxed);
            rprintln!("Delay is now 200ms");
        } else {
            BLINK_DELAY.store(del - 50_u32, Ordering::Relaxed);
            rprintln!("Delay is now {}ms", del - 50_u32);
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();
    let peripherals = embassy_nrf::init(Default::default());
    let delay_but = Input::new(peripherals.P0_14, Pull::Up);
    {
        *(BUTTON.lock().await) = Some(delay_but);
    }
    let _led_gnd = Output::new(peripherals.P0_31, Level::Low, OutputDrive::Standard);
    let mut leds: [Output; 5] = [
        Output::new(peripherals.P0_21, Level::Low, OutputDrive::Standard),
        Output::new(peripherals.P0_22, Level::Low, OutputDrive::Standard),
        Output::new(peripherals.P0_15, Level::Low, OutputDrive::Standard),
        Output::new(peripherals.P0_24, Level::Low, OutputDrive::Standard),
        Output::new(peripherals.P0_19, Level::Low, OutputDrive::Standard),
    ];

    spawner.spawn(press_button(&BUTTON)).unwrap();

    loop {
        for led in &mut leds {
            led.set_high();
            Timer::after_millis(BLINK_DELAY.load(Ordering::Relaxed) as u64).await;
            led.set_low();
            Timer::after_millis(100).await;
        }
    }
}
