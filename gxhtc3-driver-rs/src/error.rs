//! Error types for `GXHTC3` devices.

/// Error type for `GXHTC3` devices.
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
