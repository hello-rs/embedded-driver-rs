//! Chip info for the device.

/// Default device address.
const DEFAULT_ADDRESS: u8 = 0x44;

/// Register addresses for the device.
#[repr(u8)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Register {
    ChipId = 0x44,
}

/// Command for the device.
#[repr(u16)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Command {
    /// Temperature and humidity measurement
    TempHumiMea = 0x2C10,
    /// Temperature measurement
    TempMea = 0xCC44,
    /// Humidity measurement
    HumiMea = 0xCC66,
}

/// Address for the device.
#[repr(u8)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Address {
    #[default]
    Default = DEFAULT_ADDRESS,
    /// User-defined address.
    Other(u8),
}
