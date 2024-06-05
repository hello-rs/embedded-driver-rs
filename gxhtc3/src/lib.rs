#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod chip_info;
mod error;

pub use crate::address::Address;
pub use crate::error::Error;
