#![no_main]
#![no_std]

use core::cell::{Cell, RefCell};

use cortex_m_rt::entry;
use critical_section::{with, Mutex};
use nrf52833_hal::{
    gpio::p0,
    gpiote::Gpiote,
    pac::{interrupt, Interrupt, Peripherals, NVIC},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0_u32));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let peripheral = Peripherals::take().unwrap();
    let gpio_p0 = p0::Parts::new(peripheral.P0);
    let pin_p0_14 = gpio_p0.p0_14.into_pullup_input();
    let gpiote = Gpiote::new(peripheral.GPIOTE);
    let channel0 = gpiote.channel0();

    channel0
        .input_pin(&pin_p0_14.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();

    with(|cs| {
        unsafe {
            NVIC::unmask(Interrupt::GPIOTE);
        }
        NVIC::unpend(Interrupt::GPIOTE);
        GPIOTE.borrow_ref_mut(cs).replace(gpiote);
        rprintln!("Press button A!");
    });

    loop {}
}

#[interrupt]
fn GPIOTE() {
    with(|cs| {
        if let Some(gpiote) = GPIOTE.borrow(cs).borrow().as_ref() {
            gpiote.channel0().reset_events();
            let mut count = COUNT.borrow(cs).get();
            count += 1;
            rprintln!("Button press count = {}", count);
            COUNT.borrow(cs).set(count);
        }
    });
}
