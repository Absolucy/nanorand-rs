#![cfg_attr(windows, deny(unsafe_code))]
#![cfg_attr(not(windows), forbid(unsafe_code))]

/// Implementation of cryptography, for CSPRNGs.
pub mod crypto;
/// Sources for obtaining entropy.
#[cfg(feature = "std")]
pub mod entropy;
/// Traits for generating types from an RNG.
pub mod gen;
/// RNG algorithms.
pub mod rand;

pub use gen::*;
pub use rand::*;
