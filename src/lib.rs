#![cfg_attr(windows, deny(unsafe_code))]
#![cfg_attr(not(windows), forbid(unsafe_code))]

/// Sources for obtaining entropy.
#[cfg(feature = "std")]
pub mod entropy;
/// Traits for generating types from an RNG.
pub mod gen;
/// RNG algorithms.
pub mod rand;

pub use gen::*;
pub use rand::*;

#[cfg(feature = "atomics")]
use core::sync::atomic::{AtomicU64, Ordering};
#[cfg(feature = "atomics")]
static RNG_STATE_GLOBAL: AtomicU64 = AtomicU64::new(42);

/// Seed the global RNG state with a 64-bit number.
#[cfg(feature = "atomics")]
pub fn seed_global(seed: u64) {
	RNG_STATE_GLOBAL.store(seed, Ordering::Relaxed);
}

#[cfg(all(feature = "ctor", feature = "atomics"))]
#[used]
#[cfg_attr(
	any(target_os = "linux", target_os = "android", target_os = "freebsd"),
	link_section = ".init_array"
)]
#[cfg_attr(target_os = "openbsd", link_section = ".ctors")]
#[cfg_attr(
	any(target_os = "macos", target_os = "ios"),
	link_section = "__DATA,__mod_init_func"
)]
#[cfg_attr(windows, link_section = ".CRT$XCU")]
static INITIALIZE_RAND_IN_CTOR: extern "C" fn() = {
	#[cfg_attr(
		any(target_os = "linux", target_os = "android"),
		link_section = ".text.startup"
	)]
	extern "C" fn ctor() {
		seed_global(entropy::entropy_from_system());
	}
	ctor
};
