#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use nrf52833_hal::{
    pac::{interrupt, Interrupt, Peripherals, NVIC, TIMER0},
    timer::Instance,
};
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

static G_TIMER: Mutex<RefCell<Option<TIMER0>>> = Mutex::new(RefCell::new(None));
static G_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

impl Time {
    fn show_time(&self) {
        rprintln!(
            "Elapsed Time: {:0>2}:{:0>2}:{:0>2}",
            self.hours,
            self.minutes,
            self.seconds
        );
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let peripherals = Peripherals::take().expect("Couldn't find the board.");
    let timer_0 = peripherals.TIMER0;

    timer_0.set_periodic();
    timer_0.timer_start(1_000_000_u32);
    timer_0.enable_interrupt();
    timer_0.timer_reset_event();

    free(|cs| {
        unsafe {
            NVIC::unmask(Interrupt::TIMER0);
        }
        NVIC::unpend(Interrupt::TIMER0);
        G_TIMER.borrow(cs).borrow_mut().replace(timer_0);
    });

    let mut time = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    loop {
        free(|cs| {
            if G_FLAG.borrow(cs).get() {
                G_FLAG.borrow(cs).set(false);
                time.seconds += 1;
                if time.seconds > 59 {
                    time.minutes += 1;
                    time.seconds = 0;
                }
                if time.minutes > 59 {
                    time.hours += 1;
                    time.minutes = 0;
                }
                if time.hours > 23 {
                    time.seconds = 0;
                    time.minutes = 0;
                    time.hours = 0;
                }
                time.show_time();
            }
        });
    }
}

#[interrupt]
fn TIMER0() {
    free(|cs| {
        G_TIMER
            .borrow(cs)
            .borrow_mut()
            .as_ref()
            .unwrap()
            .timer_reset_event();
        G_FLAG.borrow(cs).set(true);
    });
}
