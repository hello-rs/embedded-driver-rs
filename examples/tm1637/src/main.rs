#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Ticker};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, embassy, gpio::IO, peripherals::Peripherals, prelude::*, timer::TimerGroup,
};
use log::info;
use tm1637::demo::Demo;
use tm1637_embedded_hal::asynch::TM1637;

// global unix time.
static UNIX: AtomicU32 = AtomicU32::new(0);

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    // init global info.

    // init chip system info.
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    // init chip io info.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let clk = io.pins.gpio6.into_open_drain_output();
    let dio = io.pins.gpio7.into_open_drain_output();
    // init embassy.
    embassy::init(&clocks, TimerGroup::new_async(peripherals.TIMG0, &clocks));

    // init logic info.
    let delay = Delay {};
    let mut tm = TM1637::builder(clk, dio, delay).build();
    // Initialize the display.
    // Clear the display and set the initial brightness.
    tm.init().await.unwrap();

    let delay = Delay {};
    let mut demo = Demo::new(tm, delay);

    // start task.
    spawner.spawn(update_unix()).ok();
    loop {
        let unix = UNIX.load(Ordering::SeqCst);
        demo.timer(unix).await.unwrap();
    }
}

/// 每秒更新一次unix时间.
#[embassy_executor::task]
async fn update_unix() {
    UNIX.store(1717171200, Ordering::SeqCst);
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        ticker.next().await;
        let unix = UNIX.load(Ordering::SeqCst) + 1;
        // todo 如果 unix = 0 则从网络获取时间.
        // ticker.reset();
        UNIX.store(unix, Ordering::SeqCst);
        info!("now unix is {}", unix);
    }
}
