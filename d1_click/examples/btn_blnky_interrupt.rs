#![no_main]
#![no_std]

use core::cell::{Cell, RefCell};

use cortex_m_rt::entry;
use critical_section::{with, Mutex};
use embedded_hal::{delay::DelayNs, digital::StatefulOutputPin};
use nrf52833_hal::{
    gpio::{p0, Level},
    gpiote::Gpiote,
    pac::{interrupt, Interrupt, Peripherals, NVIC},
    timer,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static GPIOTE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static BLNK_DLY: Mutex<Cell<u32>> = Mutex::new(Cell::new(1_000_u32));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let peripherals = Peripherals::take().unwrap();
    let mut timer0 = timer::Timer::new(peripherals.TIMER0);
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let _pin_p0_19 = gpio_p0.p0_19.into_push_pull_output(Level::High);
    let mut pin_p0_30 = gpio_p0.p0_30.into_push_pull_output(Level::Low);
    let pin_p0_14 = gpio_p0.p0_14.into_pullup_input();
    let gpiote = Gpiote::new(peripherals.GPIOTE);
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

    loop {
        with(|cs| {
            timer0.delay_ms(BLNK_DLY.borrow(cs).get());
            pin_p0_30.toggle().unwrap();
        });
    }
}

#[interrupt]
fn GPIOTE() {
    with(|cs| {
        rprintln!("Button A pressed!");
        if let Some(gpiote) = GPIOTE.borrow(cs).borrow().as_ref() {
            gpiote.channel0().reset_events();
            let mut blnk_dly = BLNK_DLY.borrow(cs).get();
            blnk_dly -= 250_u32;
            if blnk_dly < 250_u32 {
                blnk_dly = 1_000_u32;
            };
            BLNK_DLY.borrow(cs).replace(blnk_dly);
            rprintln!("Button A press processed! Delay: {}", blnk_dly);
        }
    });
}
