//! Register addresses for the device.

use crate::error::DHT22Error;
use embedded_hal_async::i2c::I2c;

/// Register addresses for the device.
#[repr(u16)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Register {
    ChipId = 0x0D,
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
