//! Error types for the devices.

/// Error type for the devices.
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum QMC5883LError<I2CError> {
    /// I2C error.
    I2C(I2CError),
    /// Invalid device ID.
    InvalidDevice(u8),
    /// Read taken from magnetometer before ready.
    NotReady,
    /// Arithmetic error, like deviding by zero, overflow, etc.
    Arithmetic,
}
