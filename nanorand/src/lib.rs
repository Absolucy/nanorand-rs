#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(missing_docs)]

//! A library meant for fast, random number generation with quick compile time, and minimal dependencies.
//!
//! # Examples
//! ## Generating a number with an initialized RNG
//! ```rust
//! use nanorand::{RNG, WyRand};
//!
//! let mut rng = WyRand::new();
//! println!("Random number: {}", rng.generate::<u64>());
//! ```
//! ## Generating a number with a thread-local RNG
//! ```rust
//! use nanorand::RNG;
//!
//! let mut rng = nanorand::tls_rng();
//! println!("Random number: {}", rng.generate::<u64>());
//! ```
//! ## Generating a number in a range
//! ```rust
//! use nanorand::{RNG, WyRand};
//!
//! let mut rng = WyRand::new();
//! println!("Random number between 1 and 100: {}", rng.generate_range::<u64>(1, 100));
//! ```
//! ## Shuffling a Vec
//! ```rust
//! use nanorand::{RNG, WyRand};
//!
//! let mut rng = WyRand::new();
//! let mut items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//! rng.shuffle(&mut items);
//! ```
//!
//! ## Why should I use this over...
//!
//! * `rand` - The standard rand crate is a complex beast. It contains unsafe code in the core implementations, and while it has much more options than we do, that's kind of the point. We're straight to the point, while rand is everything and the kitchen sink.
//! * `fastrand`, `oorandom`, `random-fast-rng`, or `randomize` - These are all minimal, zero-dep implementations of the PCG family of RNGs (Pcg32 and Pcg64). While these are decent, they are _much_ slower than wyrand (which beats the speed of these Pcg32 implementations while providing 64 random bits), and do not provide CSPRNGs.
//! * `getrandom` - The getrandom crate just provides OS entropy sources. It is not meant for random number generation. In fact, we provide it as an optional entropy source.
//!
//! ## RNG Implementations
//!
//! **RNG**|**nanorand type**|**Output Size**|**Cryptographically Secure**|**Speed**<sup>1</sup>|**Notes**|**Original Implementation**
//! :-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:
//! wyrand|[nanorand::WyRand](rand/wyrand/struct.WyRand.html), [nanorand::tls::TlsWyRand](tls/fn.tls_rng.html)|64 bits (`u64`)|🚫|10.09 GB/s||[https://github.com/lemire/testingRNG/blob/master/source/wyrand.h](https://github.com/lemire/testingRNG/blob/master/source/wyrand.h)
//! Pcg64|[nanorand::Pcg64](rand/pcg64/struct.Pcg64.html)|64 bits (`u64`)|🚫|2.3 GB/s||[https://github.com/rkern/pcg64](https://github.com/rkern/pcg64)
//! ChaCha|[nanorand::ChaCha](rand/chacha/struct.ChaCha.html)|512 bits (`[u32; 16]`)|✅|150 MB/s (ChaCha8), 70 MB/s (ChaCha20)|Only works in Rust 1.47 or above|[https://cr.yp.to/chacha.html](https://cr.yp.to/chacha.html)
//!
//! <sup>1. Speed benchmarked on an Intel Core i7 8086k processor running at 5.1 GHz</sup>
//!
//! ## Entropy Sources
//! _Listed in order of priority_
//!
//! * If the `getrandom` feature is enabled, then [getrandom::getrandom](https://docs.rs/getrandom/*/getrandom/fn.getrandom.html) will be called.
//! * If the `rdseed` feature is enabled, and is running on an x86(-64) system with the [RDSEED](https://en.wikipedia.org/wiki/RDRAND) instruction, then
//!   we will attempt to source as much entropy as possible via our [rdseed_entropy](entropy::rdseed_entropy) function
//! * Linux and Android will attempt to use the [`getrandom`](https://man7.org/linux/man-pages/man2/getrandom.2.html) syscall.
//! * macOS and iOS (Darwin-based systems) will use Security.framework's [`SecRandomCopyBytes`](https://developer.apple.com/documentation/security/1399291-secrandomcopybytes).
//! * Windows
//!   * If we're targeting UWP, then the [`BCryptGenRandom`](https://docs.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptgenrandom) is used with system-preferred RNG (`BCRYPT_USE_SYSTEM_PREFERRED_RNG`).
//!   * Otherwise, we'll use [`RtlGenRandom`](https://docs.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-rtlgenrandom).
//! * If all else fails, and the `std` feature is enabled, we'll resort to pulling bytes from the current system unix time ([entropy::emergency_system_time_entropy]), and screwing with them via XOR and endianness operations.
//!
//! ## Feature Flags
//!
//! * `std` (default) - Enables Rust `std` lib features, such as seeding from OS entropy sources.
//! * `tls` (default) - Enables a thread-local WyRand RNG (see below). Requires `tls` to be enabled.
//! * `wyrand` (default) - Enable the [wyrand](rand/wyrand/struct.WyRand.html) RNG.
//! * `pcg64` (default) - Enable the [Pcg64](rand/pcg64/struct.Pcg64.html)  RNG.
//! * `chacha` - Enable the [ChaCha](rand/chacha/struct.ChaCha.html) RNG. Requires Rust 1.47 or later.
//! * `rdseed` - On x86/x86_64 platforms, the `rdseed` intrinsic will be used when OS entropy isn't available.
//! * `zeroize` - Implement the [Zeroize](https://crates.io/crates/zeroize) trait for all RNGs.
//! * `getrandom` - Use the [`getrandom`](https://crates.io/crates/getrandom) crate as an entropy source.
//! Works on most systems, optional due to the fact that it brings in more dependencies.

pub use gen::*;
pub use rand::*;
#[cfg(feature = "tls")]
pub use tls::tls_rng;

/// Implementation of cryptography, for CSPRNGs.
pub mod crypto;
/// Sources for obtaining entropy.
pub mod entropy;
/// Traits for generating types from an RNG.
pub mod gen;
/// RNG algorithms.
pub mod rand;
#[cfg(feature = "tls")]
/// Provides a thread-local [WyRand] RNG.
pub mod tls;
