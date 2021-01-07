// Based off lemire's wyrand C++ code at https://github.com/lemire/testingRNG/blob/master/source/wyrand.h

use crate::RNG;
use core::fmt::{self, Display, Formatter};
#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// An instance of the WyRand random number generator.
/// Seeded from the system entropy generator when available.
/// **This generator is _NOT_ cryptographically secure.**
/// #[repr(transparent)]
#[cfg_attr(feature = "zeroize", derive(Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
#[repr(transparent)]
pub struct WyRand {
	seed: u64,
}

impl WyRand {
	/// Create a new [`WyRand`] instance, seeding from the system's default source of entropy.
	pub fn new() -> Self {
		Self::default()
	}

	/// Create a new [`WyRand`] instance, using a provided seed.
	pub fn new_seed(seed: u64) -> Self {
		Self { seed }
	}
}

impl Default for WyRand {
	/// Create a new [`WyRand`] instance, seeding from the system's default source of entropy.
	fn default() -> Self {
		let mut entropy: [u8; core::mem::size_of::<u64>()] = Default::default();
		crate::entropy::entropy_from_system(&mut entropy);
		Self {
			seed: u64::from_ne_bytes(entropy),
		}
	}
}

impl RNG for WyRand {
	type Output = [u8; 8];

	fn rand(&mut self) -> Self::Output {
		self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);
		let t: u128 = (self.seed as u128).wrapping_mul((self.seed ^ 0xe7037ed1a0b428db) as u128);
		let ret = ((t >> 64) ^ t) as u64;
		ret.to_ne_bytes()
	}

	fn rand_with_seed(seed: &[u8]) -> Self::Output {
		let mut seed_bytes = [0u8; 8];
		seed_bytes.iter_mut().zip(seed).for_each(|(a, b)| *a = *b);
		let seed = u64::from_ne_bytes(seed_bytes);
		let seed = seed.wrapping_add(0xa0761d6478bd642f);
		let t: u128 = (seed as u128).wrapping_mul((seed ^ 0xe7037ed1a0b428db) as u128);
		let ret = ((t >> 64) ^ t) as u64;
		ret.to_ne_bytes()
	}

	fn reseed(&mut self, new_seed: &[u8]) {
		let mut seed = [0u8; 8];
		seed.iter_mut().zip(new_seed).for_each(|(a, b)| *a = *b);
		self.seed = u64::from_ne_bytes(seed)
	}
}

impl Clone for WyRand {
	fn clone(&self) -> Self {
		Self { seed: self.seed }
	}
}

impl Display for WyRand {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "WyRand ({:p})", self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// TDD for https://github.com/aspenluxxxy/nanorand-rs/issues/21
	#[test]
	fn generate_range_contains_upper_test() {
		generate_range_contains_upper(0, 1);
		generate_range_contains_upper(1000, 1010);
		generate_range_contains_upper(u64::MAX - 1, u64::MAX);
	}

	fn generate_range_contains_upper(lower: u64, upper: u64) {
		type T = u64;
		let n = (((upper - lower) + 1) * 1000) as usize;
		let mut rng = WyRand::new();
		let mut buff: Vec<T> = Vec::with_capacity(n);
		for _ in 0..n {
			buff.push(rng.generate_range::<T>(lower, upper));
		}
		// we should get non-zero amount of `upper` and `lower` values by pure chance..
		assert!(buff.contains(&lower));
		assert!(buff.contains(&upper));
	}
}
