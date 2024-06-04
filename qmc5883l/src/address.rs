//! The device I2C address.

/// Default I2C address.
const DEFAULT_ADDRESS: u8 = 0x0D;

/// The device I2C address.
#[repr(u8)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum I2cAddress {
    /// Default I2C address.
    #[default]
    Default = DEFAULT_ADDRESS,
    /// User-defined I2C address.
    Other(u8),
}

impl From<I2cAddress> for u8 {
    fn from(address: I2cAddress) -> u8 {
        match address {
            I2cAddress::Default => DEFAULT_ADDRESS,
            I2cAddress::Other(addr) => addr,
        }
    }
}

impl From<u8> for I2cAddress {
    fn from(addr: u8) -> I2cAddress {
        match addr {
            DEFAULT_ADDRESS => I2cAddress::Default,
            addr => I2cAddress::Other(addr),
        }
    }
}
