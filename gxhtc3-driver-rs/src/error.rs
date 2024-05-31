//! Error types for the devices.

/// Error type for the devices.
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub enum GXHTC3Error<I2CError> {
    /// I2C error.
    I2C(I2CError),
    /// Invalid device ID.
    InvalidId(u8),
    /// Arithmetic error, like deviding by zero, overflow, etc.
    Arithmetic,
}
