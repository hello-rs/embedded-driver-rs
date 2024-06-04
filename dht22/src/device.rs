//! Device definition and implementation.

use crate::{
    address::I2cAddress,
    error::DHT22Error,
    register::{ChipId, Register},
};
use embedded_hal_async::i2c::I2c;

/// `DHT22` builder.
#[derive(Clone)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub struct DHT22Builder<I2C> {
    inner: DHT22<I2C>,
}

impl<I2C> DHT22Builder<I2C>
where
    I2C: I2c,
{
    /// Create a new builder.
    pub fn new(i2c: I2C) -> Self {
        Self {
            inner: DHT22 {
                addr: I2cAddress::default(),
                i2c,
            },
        }
    }

    /// Set the device address.
    pub fn addr(mut self, addr: I2cAddress) -> Self {
        self.inner.addr = addr;
        self
    }

    /// Read device ID.
    async fn read_id(&mut self) -> Result<u8, I2C::Error> {
        let mut data = [0u8; 2];
        self.inner
            .i2c
            .write_read(self.inner.addr.into(), &[Register::ChipId as u8], &mut data)
            .await?;
        Ok(data[0])
    }

    /// Build the device.
    pub async fn build(mut self) -> Result<DHT22<I2C>, DHT22Error<I2C::Error>> {
        let id = self.read_id().await.map_err(DHT22Error::I2C)?;
        if !ChipId::valid(id) {
            return Err(DHT22Error::InvalidDevice(id));
        }
        Ok(self.inner)
    }
}
/// `DHT22` device.
#[derive(Clone)]
#[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
#[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
pub struct DHT22<I2C> {
    addr: I2cAddress,
    i2c: I2C,
}
impl<I2C> DHT22<I2C>
where
    I2C: I2c,
{
    /// Create a new builder.
    pub fn builder(i2c: I2C) -> DHT22Builder<I2C> {
        DHT22Builder::new(i2c)
    }
}