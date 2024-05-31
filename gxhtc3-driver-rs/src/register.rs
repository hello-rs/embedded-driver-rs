//! Register addresses for the device.

/// Register addresses for the device.
#[repr(u16)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Register {
    ChipId = 0xEFC8,
    Sleep = 0xB098,
    Wakeup = 0x3517,
    Control = 0xF4,
    /// Temperature and humidity measurement command
    TempHumiMeaCmd = 0x7CA2,
}
