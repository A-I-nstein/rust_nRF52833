#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_executor::Spawner;
use embassy_nrf::{
    config::Config,
    gpio::{Input, Pull},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

type ButtonType = Mutex<CriticalSectionRawMutex, Option<Input<'static>>>;

static BUTTON: ButtonType = Mutex::new(None);
static PRESS_COUNTER: AtomicU32 = AtomicU32::new(0_u32);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();
    let p = embassy_nrf::init(Config::default());
    let counter_button = Input::new(p.P0_14, Pull::Up);

    *BUTTON.lock().await = Some(counter_button);
    spawner.spawn(press_button(&BUTTON)).unwrap();
}

#[embassy_executor::task]
async fn press_button(button: &'static ButtonType) {
    loop {
        let mut button_unlocked = button.lock().await;
        if let Some(button_ref) = button_unlocked.as_mut() {
            button_ref.wait_for_falling_edge().await;
            let mut count = PRESS_COUNTER.load(Ordering::Relaxed);
            count += 1;
            PRESS_COUNTER.store(count, Ordering::Relaxed);
            rprintln!("Button pressed {} times", count);
        }
    }
}
