#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*};


#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    embassy::init(&clocks, TimerGroup::new_async(peripherals.TIMG0, &clocks));
    spawner.spawn(run()).ok();

    loop {
        log::info!("Hello world!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}


#[embassy_executor::task]
async fn run() {
    loop {
        println!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
