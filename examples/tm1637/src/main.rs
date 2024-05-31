#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy::{self},
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
};
use log::info;

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    embassy::init(&clocks, TimerGroup::new_async(peripherals.TIMG0, &clocks));
    spawner.spawn(run()).ok();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

#[embassy_executor::task]
async fn run() {
    loop {
        info!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
