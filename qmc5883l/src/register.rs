//! Register addresses for the device.

use embedded_hal_async::i2c::I2c;

use crate::error::QMC5883LError;

/// Register addresses for the device.
#[repr(u16)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Register {
    DataOutputXLSB = 0x00,
    DataOutputXMSB = 0x01,
    DataOutputYLSB = 0x02,
    DataOutputYMSB = 0x03,
    DataOutputZLSB = 0x04,
    DataOutputZMSB = 0x05,
    Status = 0x06,
    ///  Temperature Data
    TempLSB = 0x07,
    TempMSB = 0x08,
    /// OSR[7:6](Over Sample Ratio)
    /// RNG[5:4](Full Scale)
    /// ODR[3:2](Output Data Rate)
    /// MODE[1:0](Mode Control)
    Control1 = 0x09,
    /// SOFT_RST[7](Soft Reset)
    /// ROL_PNT[6](Point roll over function enabling)
    /// INT_ENB[0](Interrupt Pin enabling)
    Control2 = 0x0A,
    SetReset = 0x0B,
    ChipId = 0x0D,
}

/// Update frequency.
/// For most of compassing applications, we recommend 10 Hz for low power consumption.
/// For gaming, the high update rate such as 100Hz or 200Hz can be used.
#[repr(u8)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum OutputDataRate {
    Rate10Hz = 0x0,
    Rate50Hz = 0b0100,
    Rate100Hz = 0b1000,
    Rate200Hz = 0b1100,
}

/// Oversampling rate; controls bandwidth of internal digital filter.
/// Larger OSR value leads to smaller filter bandwidth,
/// less in-band noise and higher power consumption.
/// Four over sample ratio can be selected, 64, 128, 256 or 512.
#[repr(u8)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum OversampleRate {
    Rate64 = 0b11 << 6,
    Rate128 = 0b10 << 6,
    Rate256 = 0b01 << 6,
    Rate512 = 0,
}

/// Field range of magnetic sensor.
/// For magnetic clear environment, low field range such as +/- 2gauss can be used.
/// The lowest field range has the highest sensitivity, therefore, higher resolution.
#[repr(u8)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum FieldRange {
    /// ± 2 gauss
    Range2Gauss = 0,
    /// ± 8 gauss
    Range8Gauss = 1 << 3,
}

/// Chip identification.
#[repr(u8)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum ChipId {
    Val = 0xFF,
}

impl ChipId {
    /// Validate the chip ID.
    #[inline]
    pub fn valid(id: u8) -> bool {
        id == ChipId::Val as u8
    }
}
