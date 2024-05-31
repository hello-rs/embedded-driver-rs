#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod address;
mod error;
mod register;

pub use crate::address::Address;
pub use crate::error::GXHTC3Error;
