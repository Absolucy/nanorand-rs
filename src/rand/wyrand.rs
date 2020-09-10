// Based off lemire's wyrand C++ code at https://github.com/lemire/testingRNG/blob/master/source/wyrand.h

use super::RNG;

/// An instance of the wyrand random number generator.  
/// Seeded from the system entropy generator when available.  
/// **This generator is _NOT_ cryptographically secure.**
pub struct WyRand {
	seed: u64,
}

#[cfg(feature = "std")]
impl WyRand {
	/// Create a new [`WyRand`] instance, seeding from the system's default source of entropy.
	pub fn new() -> Self {
		Self {
			seed: crate::entropy::entropy_from_system(),
		}
	}
}

#[cfg(feature = "std")]
impl Default for WyRand {
	/// Create a new [`WyRand`] instance, seeding from the system's default source of entropy.
	fn default() -> Self {
		Self {
			seed: crate::entropy::entropy_from_system(),
		}
	}
}

impl RNG for WyRand {
	fn rand(&mut self) -> u64 {
		let ret = Self::rand_with_seed(self.seed);
		self.seed = ret;
		ret
	}

	fn rand_with_seed(seed: u64) -> u64 {
		let seed = seed.wrapping_add(0xa0761d6478bd642f);
		let t: u128 = (seed as u128).wrapping_mul((seed ^ 0xe7037ed1a0b428db) as u128);
		let ret = ((t >> 64) ^ t) as u64;
		ret
	}

	fn reseed(&mut self, new_seed: u64) {
		self.seed = new_seed;
	}
}

impl Clone for WyRand {
	fn clone(&self) -> Self {
		Self { seed: self.seed }
	}
}
