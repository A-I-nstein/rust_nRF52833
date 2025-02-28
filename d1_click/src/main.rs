#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use microbit::{
    hal::gpiote::Gpiote,
    pac::{interrupt, Interrupt, NVIC},
    Board,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().expect("Could not find board.");
    let gpiote = Gpiote::new(board.GPIOTE);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board.buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();
    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board.buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    cortex_m::interrupt::free(move |cs| {
        unsafe {
            NVIC::unmask(Interrupt::GPIOTE);
        }
        NVIC::unpend(Interrupt::GPIOTE);
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);
        rprintln!("Press one or both the buttons!");
    });

    loop {
        continue;
    }
}

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {

            for _ in 0..100000 { // Adjust the loop count as needed
                cortex_m::asm::nop(); // A no-operation instruction
            }

            let buttonapressed = gpiote.channel0().is_event_triggered();
            let buttonbpressed = gpiote.channel1().is_event_triggered();

            let buttons_pressed = match (buttonapressed, buttonbpressed) {
                (false, false) => "",
                (true, false) => "A",
                (false, true) => "B",
                (true, true) => "A + B",
            };
            rprintln!("Button pressed {:?}", buttons_pressed);
            
            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
        }
    });
}
