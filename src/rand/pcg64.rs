// Based off Robert Kern's C implementation at https://github.com/rkern/pcg64/blob/master/pcg64.c

use super::RNG;

const PCG_DEFAULT_MULTIPLIER_128: u128 = 47026247687942121848144207491837523525;

/// An instance of the Pcg64 random number generator.  
/// Seeded from the system entropy generator when available.  
/// **This generator is _NOT_ cryptographically secure.**
pub struct Pcg64 {
	seed: u128,
	state: u128,
	inc: u128,
}

#[cfg(feature = "std")]
impl Pcg64 {
	/// Create a new [`Pcg64`] instance, seeding from the system's default source of entropy.
	pub fn new() -> Self {
		Self {
			seed: (crate::entropy::entropy_from_system() as u128).rotate_right(64)
				| crate::entropy::entropy_from_system() as u128,
			inc: 0,
			state: 0,
		}
	}

	#[inline]
	fn step(&mut self) {
		self.state = self
			.state
			.wrapping_mul(PCG_DEFAULT_MULTIPLIER_128)
			.wrapping_add(self.inc);
	}

	#[inline]
	fn rand128(&mut self) -> u64 {
		self.state = 0;
		self.inc = self.seed.rotate_left(1) | 1;
		self.step();
		self.state = self.state.wrapping_add(self.seed);
		self.step();
		self.step();
		(((self.state >> 64) as u64) ^ (self.state as u64)) >> (self.state >> 122)
	}

	pub fn reseed128(&mut self, seed: u128) {
		self.seed = seed;
	}
}

#[cfg(feature = "std")]
impl Default for Pcg64 {
	/// Create a new [`Pcg64`] instance, seeding from the system's default source of entropy.
	fn default() -> Self {
		Self {
			seed: (crate::entropy::entropy_from_system() as u128 >> 64)
				| crate::entropy::entropy_from_system() as u128,
			inc: 0,
			state: 0,
		}
	}
}

impl RNG for Pcg64 {
	fn rand(&mut self) -> u64 {
		let ret = self.rand128();
		self.seed = self.state ^ (ret as u128).rotate_right(64);
		ret
	}

	fn rand_with_seed(_seed: u64) -> u64 {
		unimplemented!("Pcg64 requires a state!");
	}

	fn reseed(&mut self, new_seed: u64) {
		self.seed = new_seed as u128;
	}
}

impl Clone for Pcg64 {
	fn clone(&self) -> Self {
		Self {
			seed: self.seed,
			inc: self.inc,
			state: self.state,
		}
	}
}
