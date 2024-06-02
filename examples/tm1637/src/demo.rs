//! Asynchronous demo module.

use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs;
use embedded_utils_rs::time::{DateTime, TimeZone};
use tm1637_embedded_hal::{asynch::TM1637, mappings::DigitBits};

/// Asynchronous demo.
pub struct Demo<CLK, DIO, DELAY, ERR>
where
    CLK: OutputPin<Error = ERR>,
    DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    device: TM1637<CLK, DIO, DELAY>,
    delay: DELAY,
}
impl<CLK, DIO, DELAY, ERR> Demo<CLK, DIO, DELAY, ERR>
where
    ERR: core::fmt::Debug,
    CLK: OutputPin<Error = ERR>,
    DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    /// Create a new demo instance.
    pub fn new(device: TM1637<CLK, DIO, DELAY>, delay: DELAY) -> Self {
        Self { device, delay }
    }

    /// Create a timer that display minute and second.
    pub async fn timer(&mut self, unix: u32) -> Result<(), ERR> {
        let date_time = DateTime::from_unix_secs(unix.into(), TimeZone::AsiaShanghai);
        let mut data: [DigitBits; 4] = [DigitBits::Zero; 4];
        // 为了演示，使用分钟和秒数来显示时间
        data[0] = DigitBits::from_digit(date_time.minute / 10 as u8);
        data[1] = DigitBits::from_digit(date_time.minute % 10 as u8);
        data[2] = DigitBits::from_digit(date_time.second / 10 as u8);
        data[3] = DigitBits::from_digit(date_time.second % 10 as u8);
        self.device
            .write_segments_raw(0, &data.map(|d| d as u8))
            .await?;
        self.delay.delay_ms(1000).await;
        Ok(())
    }
}
