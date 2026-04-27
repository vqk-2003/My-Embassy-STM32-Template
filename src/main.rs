#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// There is a known problem of failing to
// flash after the first time when using Embassy
// The current work around is to connect the NRST line
// from the STLINK to the target
// https://users.rust-lang.org/t/probe-rs-fails-to-work-after-first-time-use-successful/103234
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, World!");
    let p = embassy_stm32::init(Default::default());

    let mut led = Output::new(p.PC13, Level::Low, Speed::Low);

    loop {
        led.set_high();
        info!("High");
        Timer::after_millis(1000).await;

        led.set_low();
        info!("Low");
        Timer::after_millis(1000).await;
    }
}
