#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(any(windows, feature = "rdseed"), deny(unsafe_code))]
#![cfg_attr(not(any(windows, feature = "rdseed")), forbid(unsafe_code))]
#![forbid(missing_docs)]

//! A library meant for fast, random number generation with quick compile time, and minimal dependencies.
//!
//! # Examples
//! ## Generating a number
//! ```rs
//! use nanorand::{RNG, WyRand};
//!
//! fn main() {
//!     let mut rng = WyRand::new();
//!     println!("Random number: {}", rng.generate::<u64>());
//! }
//! ```
//! ## Generating a number in a range
//! ```rs
//! use nanorand::{RNG, WyRand};
//!
//! fn main() {
//!     let mut rng = WyRand::new();
//!     println!("Random number between 1 and 100: {}", rng.generate_range::<u64>(1, 100));
//! }
//! ```
//! ## Shuffling a Vec
//! ```rs
//! use nanorand::{RNG, WyRand};
//!
//! fn main() {
//!     let mut rng = WyRand::new();
//!     let mut items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//!     rng.shuffle(&mut items);
//! }
//! ```
//!
//! ## Why should I use this over...
//!
//! * `rand` - The standard rand crate is a complex beast. It contains unsafe code in the core implementations, and while it has much more options than we do, that's kind of the point. We're straight to the point, while rand is everything and the kitchen sink.  
//! * `fastrand`, `oorandom`, `random-fast-rng`, or `randomize` - These are all minimal, zero-dep implementations of the PCG family of RNGs (Pcg32 and Pcg64). While these are decent, they are _much_ slower than wyrand (which beats the speed of these Pcg32 implementations while providing 64 random bits), and do not provide CSPRNGs.  
//! * `getrandom` - The getrandom crate just provides OS entropy sources. It is not meant for random number generation. In fact, it is useful for seeding nanorand's RNGs on platforms where we can't do that ourselves.
//!
//! ## RNG Implementations
//!
//! **RNG**|**nanorand type**|**Output Size**|**Cryptographically Secure**|**Speed**|**Notes**|**Original Implementation**
//! :-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:
//! wyrand|[nanohash::WyRand](rand/wyrand/struct.WyRand.html)|64 bits (`u64`)|🚫|4 GB/s||https://github.com/lemire/testingRNG/blob/master/source/wyrand.h
//! Pcg64|[nanohash::Pcg64](rand/pcg64/struct.Pcg64.html)|64 bits (`u64`)|🚫|1 GB/s||https://github.com/rkern/pcg64
//! ChaCha|[nanohash::ChaCha](rand/chacha/struct.ChaCha.html)|512 bits (`[u32; 16]`)|✅|90 MB/s (ChaCha8), 40 MB/s (ChaCha20)|Currently only works in **Nightly** Rust, will work with Stable 1.47 (see [rust#74060](https://github.com/rust-lang/rust/pull/74060))|https://cr.yp.to/chacha.html
//!  
//! ## Entropy Sources
//!
//! * Unix-like (Linux, Android, macOS, iOS, FreeBSD, OpenBSD) - first `/dev/urandom`, else `/dev/random`, else system time. (`#[forbid(unsafe_code)]`)
//! * Windows - `BCryptGenRandom` with system-preferred RNG. (`#[deny(unsafe_code)]`)
//!
//! ## Feature Flags
//!
//! * `std` (default) - Enables Rust `std` lib features, such as seeding from OS entropy sources.  
//! * `wyrand` (default) - Enable the [wyrand](rand/wyrand/struct.WyRand.html) RNG.
//! * `pcg64` (default) - Enable the [Pcg64](rand/pcg64/struct.Pcg64.html)  RNG.
//! * `chacha` (**Nightly-only**) - Enable the [ChaCha](rand/chacha/struct.ChaCha.html) RNG.
//! * `rdseed` - On x86/x86_64 platforms, the `rdseed` intrinsic will be used when OS entropy isn't available.
//! * `zeroize` - Implement the [Zeroize](https://crates.io/crates/zeroize) trait for all RNGs.

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
