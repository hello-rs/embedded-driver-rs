#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod chip_info;
mod device;
mod error;

pub use crate::device::*;
pub use crate::error::DHT22Error;
