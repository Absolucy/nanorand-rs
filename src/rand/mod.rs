/// Implementation of the wyrand RNG algorithm.  
/// More details can be seen at https://github.com/wangyi-fudan/wyhash
#[cfg(feature = "wyrand")]
pub mod wyrand;
#[cfg(feature = "wyrand")]
pub use wyrand::WyRand;

#[cfg(feature = "pcg64")]
pub mod pcg64;
#[cfg(feature = "pcg64")]
pub use pcg64::Pcg64;

use crate::RNG_STATE_GLOBAL;
use core::sync::atomic::Ordering;

/// A trait that represents a random number generator.
/// It is expected that
pub trait RNG: Clone {
	/// Generates a random 64-bit integer, seeding from the internal state.
	fn rand(&mut self) -> u64;
	/// Generates a random 64-bit integer, with a custom seed.
	fn rand_with_seed(seed: u64) -> u64;
	/// Reseeds the RNG using a custom seed.
	fn reseed(&mut self, new_seed: u64);
	/// Generates a random 64-bit integer in a custom range, seeding from the internal state.
	fn rand_range(&mut self, lower: u64, upper: u64) -> u64 {
		let random_number = self.rand();
		(random_number % (upper - lower + 1)) + lower
	}
	/// Generates a random 64-bit integer in a custom range, with a custom seed.
	fn rand_range_with_seed(&mut self, seed: u64, lower: u64, upper: u64) -> u64 {
		let random_number = Self::rand_with_seed(seed);
		(random_number % (upper - lower + 1)) + lower
	}
	/// Generates a random 64-bit integer, seeding from the global state.  
	/// Note that _this is slower than an internal state_, due to use of atomics.
	fn rand_global() -> u64 {
		let seed = RNG_STATE_GLOBAL.load(Ordering::Acquire);
		let random_number = Self::rand_with_seed(seed);
		RNG_STATE_GLOBAL.compare_and_swap(seed, random_number, Ordering::Release);
		random_number
	}
	/// Generates a random 64-bit integer in a custom range, seeding from the global state.
	fn rand_global_range(lower: u64, upper: u64) -> u64 {
		let random_number = Self::rand_global();
		(random_number % (upper - lower + 1)) + lower
	}
}

pub trait RandomGen<R: RNG> {
	/// Return a random instance of the implementing type.
	fn generate(r: &mut R) -> Self;
}

impl<R: RNG> RandomGen<R> for u8 {
	fn generate(r: &mut R) -> Self {
		(r.rand() & 0xFF) as u8
	}
}

impl<R: RNG> RandomGen<R> for u16 {
	fn generate(r: &mut R) -> Self {
		(r.rand() & 0xFFFF) as u16
	}
}

impl<R: RNG> RandomGen<R> for u32 {
	fn generate(r: &mut R) -> Self {
		(r.rand() & 0xFFFFFFFF) as u32
	}
}

impl<R: RNG> RandomGen<R> for u64 {
	fn generate(r: &mut R) -> Self {
		r.rand()
	}
}

impl<R: RNG> RandomGen<R> for usize {
	fn generate(r: &mut R) -> Self {
		r.rand() as usize
	}
}

impl<R: RNG> RandomGen<R> for i8 {
	fn generate(r: &mut R) -> Self {
		(r.rand() & 0xFF) as i8
	}
}

impl<R: RNG> RandomGen<R> for i16 {
	fn generate(r: &mut R) -> Self {
		(r.rand() & 0xFFFF) as i16
	}
}

impl<R: RNG> RandomGen<R> for i32 {
	fn generate(r: &mut R) -> Self {
		(r.rand() & 0xFFFFFFFF) as i32
	}
}

impl<R: RNG> RandomGen<R> for i64 {
	fn generate(r: &mut R) -> Self {
		r.rand() as i64
	}
}

impl<R: RNG> RandomGen<R> for isize {
	fn generate(r: &mut R) -> Self {
		r.rand() as isize
	}
}
