#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod address;
mod device;
mod error;
mod register;

pub use crate::device::*;
pub use crate::error::DHT22Error;
