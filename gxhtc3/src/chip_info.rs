//! Chip info for the device.

/// Default device address.
const DEFAULT_ADDRESS: u8 = 0x44;

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

/// Default I2C address.
const DEFAULT_ADDRESS: u8 = 0x70;

/// The device I2C address.
#[repr(u8)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum Address {
    /// Default I2C address.
    #[default]
    Default = DEFAULT_ADDRESS,
    /// User-defined I2C address.
    Other(u8),
}

impl From<Address> for u8 {
    fn from(address: Address) -> u8 {
        match address {
            Address::Default => DEFAULT_ADDRESS,
            Address::Other(addr) => addr,
        }
    }
}

impl From<u8> for Address {
    fn from(addr: u8) -> Address {
        match addr {
            DEFAULT_ADDRESS => Address::Default,
            addr => Address::Other(addr),
        }
    }
}
