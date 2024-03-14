#![cfg_attr(not(feature = "std"), no_std)]
mod generated;
mod types;

pub use generated::*;
pub use types::{Argument, ArgumentError};
