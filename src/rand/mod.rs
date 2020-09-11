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

#[cfg(feature = "atomics")]
use crate::RNG_STATE_GLOBAL;
#[cfg(feature = "atomics")]
use core::sync::atomic::Ordering;

use crate::gen::RandomGen;

/*
	fn rand_range(&mut self, lower: u64, upper: u64) -> u64 {
		let t = ((-(upper as i64)) % (upper as i64)) as u64;
		let mut l: u64;
		let mut m: u128;
		let in_range = loop {
			let x = self.rand();
			m = (x as u128).wrapping_mul(upper as u128);
			l = m as u64;
			if l >= t {
				break (m >> 64) as u64;
			}
		};
		in_range.max(lower)
	}
*/

/// A trait that represents a random number generator.
pub trait RNG: Clone {
	type Output: AsRef<[u8]>;

	/// Generates a random sequence of bytes, seeding from the internal state.
	fn rand(&mut self) -> Self::Output;
	/// Generates a random sequence of bytes, with a custom seed.
	fn rand_with_seed(seed: &[u8]) -> Self::Output;
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate<R: RandomGen<Self>>(&mut self) -> R {
		R::generate(self)
	}
	/// Reseeds the RNG using a custom seed.
	fn reseed(&mut self, new_seed: &[u8]);
	/*
	/// Generates a random sequence of bytes, seeding from the global state.
	/// Note that _this is slower than an internal state_, due to use of atomics.
	#[cfg(feature = "atomics")]
	fn rand_global() -> &'static [u8] {
		let mut seed_bytes = [0u8; 8];
		let seed = RNG_STATE_GLOBAL.load(Ordering::Acquire);
		seed_bytes.iter_mut().zip(&seed.to_ne_bytes()).for_each(|(a, b)| *a = *b);
		let random = Self::rand_with_seed(&seed.to_ne_bytes());
		seed_bytes.iter_mut().zip(&seed.to_ne_bytes()).for_each(|(a, b)| *a = *b);
		RNG_STATE_GLOBAL.compare_and_swap(seed, random_number, Ordering::Release);
		random_number
	}
	*/
}
