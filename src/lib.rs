#![cfg_attr(any(feature = "no_std", target_family = "windows"), deny(unsafe_code))]
#![cfg_attr(
	not(any(feature = "no_std", target_family = "windows")),
	forbid(unsafe_code)
)]

pub mod entropy;
pub mod rand;

use core::sync::atomic::{AtomicU64, Ordering};

pub use rand::wyrand::rand_global as rand;

static RNG_STATE_GLOBAL: AtomicU64 = AtomicU64::new(42);

pub fn seed_global(seed: u64) {
	RNG_STATE_GLOBAL.store(seed, Ordering::Relaxed);
}

#[cfg(feature = "ctor")]
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
