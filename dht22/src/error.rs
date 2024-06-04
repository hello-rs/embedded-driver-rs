//! Error types for the devices.

use core::fmt;

/// Error type for the devices.
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum DHT22Error<I2CError> {
    /// I2C error.
    I2C(I2CError),
    /// Invalid device ID.
    InvalidDevice(u8),
    /// Read taken from magnetometer before ready.
    NotReady,
    /// Arithmetic error, like deviding by zero, overflow, etc.
    Arithmetic,
}

impl<HE: fmt::Debug> fmt::Display for DHT22Error<HE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotPresent => write!(f, "DHT device not found"),
            ChecksumMismatch(expected, calculated) => write!(
                f,
                "Data read was corrupt (expected checksum {:x}, calculated {:x})",
                expected, calculated
            ),
            InvalidData => f.write_str("Received data is out of range"),
            Timeout => f.write_str("Timed out waiting for a read"),
            I2C(err) => write!(f, "HAL pin error: {:?}", err),
        }
    }
}
