#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(any(windows, feature = "rdseed"), deny(unsafe_code))]
#![cfg_attr(not(any(windows, feature = "rdseed")), forbid(unsafe_code))]

/// Implementation of cryptography, for CSPRNGs.
pub mod crypto;
/// Sources for obtaining entropy.
#[cfg(feature = "std")]
#[cfg_attr(any(windows, feature = "rdseed"), allow(unsafe_code))]
pub mod entropy;
/// Traits for generating types from an RNG.
pub mod gen;
/// RNG algorithms.
pub mod rand;

pub use gen::*;
pub use rand::*;
