#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use nrf52833_hal::{
    gpio::p0,
    gpiote::Gpiote,
    pac::{interrupt, Interrupt, Peripherals, NVIC, TIMER0},
    timer::Instance,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static GPIOTE_OBJ: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static G_TIMER: Mutex<RefCell<Option<TIMER0>>> = Mutex::new(RefCell::new(None));
static TIMER_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static IS_TIMER_RUNNING: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static TIMER_RESET_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

struct TimerUi {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

impl TimerUi {
    fn new() -> Self {
        return TimerUi {
            seconds: 0_u32,
            minutes: 0_u32,
            hours: 0_u32,
        };
    }

    fn increment(&mut self) {
        self.seconds += 1;
        if self.seconds > 59 {
            self.minutes += 1;
            self.seconds = 0;
        }
        if self.minutes > 59 {
            self.hours += 1;
            self.minutes = 0;
        }
        if self.hours > 23 {
            self.reset();
        }
    }

    fn show_time(&self) {
        rprintln!(
            "Elapsed Time: {:0>2}:{:0>2}:{:0>2}",
            self.hours,
            self.minutes,
            self.seconds
        );
    }

    fn reset(&mut self) {
        self.seconds = 0;
        self.minutes = 0;
        self.hours = 0;
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = Peripherals::take().expect("Could not find the board!");
    let gpio_p0 = p0::Parts::new(peripherals.P0);
    let start_stop_button = gpio_p0.p0_23.into_pullup_input();
    let reset_button = gpio_p0.p0_14.into_pullup_input();
    let gpiote = Gpiote::new(peripherals.GPIOTE);
    let channel_0 = gpiote.channel0();
    let channel_1 = gpiote.channel1();
    let timer_0 = peripherals.TIMER0;
    let mut time = TimerUi::new();

    channel_0
        .input_pin(&start_stop_button.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel_0.reset_events();
    channel_1
        .input_pin(&reset_button.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel_1.reset_events();

    free(|cs| {
        unsafe {
            NVIC::unmask(Interrupt::GPIOTE);
            NVIC::unmask(Interrupt::TIMER0);
        }
        NVIC::unpend(Interrupt::GPIOTE);
        GPIOTE_OBJ.borrow(cs).borrow_mut().replace(gpiote);
        NVIC::unpend(Interrupt::TIMER0);
        G_TIMER.borrow(cs).borrow_mut().replace(timer_0);
        rprintln!("Press button B to start/stop and button A to reset.");
    });

    loop {
        free(|cs| {
            if TIMER_FLAG.borrow(cs).get() {
                TIMER_FLAG.borrow(cs).set(false);
                time.increment();
                time.show_time();
            }
            if TIMER_RESET_FLAG.borrow(cs).get() {
                TIMER_RESET_FLAG.borrow(cs).set(false);
                IS_TIMER_RUNNING.borrow(cs).set(false);
                time.reset();
            }
        });
    }
}

#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = GPIOTE_OBJ.borrow(cs).borrow().as_ref() {
            if let Some(timer_0) = G_TIMER.borrow(cs).borrow().as_ref() {
                if gpiote.channel0().is_event_triggered() {
                    if IS_TIMER_RUNNING.borrow(cs).get() {
                        IS_TIMER_RUNNING.borrow(cs).set(false);
                        rprintln!("Timer stopped.");
                        timer_0.timer_cancel();
                    } else {
                        rprintln!("Timer started.");
                        timer_0.set_periodic();
                        timer_0.timer_start(1_000_000_u32);
                        timer_0.enable_interrupt();
                        timer_0.timer_reset_event();
                        IS_TIMER_RUNNING.borrow(cs).set(true);
                    }
                    gpiote.channel0().reset_events();
                } else if gpiote.channel1().is_event_triggered() {
                    rprintln!("Timer Restarted.");
                    timer_0.timer_cancel();
                    TIMER_RESET_FLAG.borrow(cs).set(true);
                    gpiote.channel1().reset_events();
                }
            }
        }
    });
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
        TIMER_FLAG.borrow(cs).set(true);
    });
}
