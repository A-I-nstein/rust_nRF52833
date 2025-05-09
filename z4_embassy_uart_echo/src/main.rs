#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    peripherals::UARTE0,
    uarte::{self, UarteRx, UarteTx},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const READ_BUF_SIZE: usize = 8;
static DATAPIPE: Pipe<CriticalSectionRawMutex, READ_BUF_SIZE> = Pipe::new();

bind_interrupts!(struct Irqs {
    UARTE0 => uarte::InterruptHandler<UARTE0>;
});

#[embassy_executor::task]
async fn uart_writer(mut tx: UarteTx<'static, UARTE0>) {
    let mut wbuf: [u8; READ_BUF_SIZE] = [0u8; READ_BUF_SIZE];
    loop {
        rprintln!("Console writing");
        DATAPIPE.read(&mut wbuf).await;
        rprintln!("Sending Letter");
        tx.write(&wbuf).await.unwrap();
        rprintln!("Sending New Line");
        tx.write(&[0x0D, 0x0A]).await.unwrap();
    }
}

#[embassy_executor::task]
async fn uart_reader(mut rx: UarteRx<'static, UARTE0>) {
    let mut rbuf: [u8; READ_BUF_SIZE] = [0u8; READ_BUF_SIZE];
    loop {
        rprintln!("Console reading");
        let r = rx.read(&mut rbuf).await;
        match r {
            Ok(_len) => {
                DATAPIPE.write_all(&rbuf).await;
            }
            Err(e) => {
                rprintln!("RX Error: {:?}", e);
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();
    let p = embassy_nrf::init(Default::default());

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;
    let uart = uarte::Uarte::new(p.UARTE0, Irqs, p.P1_08, p.P0_06, config);
    rprintln!("uarte initialized!");

    let (tx, rx) = uart.split();

    spawner.spawn(uart_reader(rx)).ok();
    spawner.spawn(uart_writer(tx)).ok();
}
