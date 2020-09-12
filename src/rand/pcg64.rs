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

impl Pcg64 {
	/// Create a new [`Pcg64`] instance, seeding from the system's default source of entropy.
	#[cfg(feature = "std")]
	pub fn new() -> Self {
		let mut entropy: [u8; std::mem::size_of::<u128>()] = Default::default();
		entropy.copy_from_slice(&crate::entropy::entropy_from_system(std::mem::size_of::<
			u128,
		>()));
		Self {
			seed: u128::from_ne_bytes(entropy),
			inc: 0,
			state: 0,
		}
	}

	/// Create a new [`Pcg64`] instance, using a provided seed.
	pub fn new_seed(seed: u128) -> Self {
		Self {
			seed,
			inc: 0,
			state: 0,
		}
	}

	#[inline(always)]
	fn step(&mut self) {
		self.state = self
			.state
			.wrapping_mul(PCG_DEFAULT_MULTIPLIER_128)
			.wrapping_add(self.inc);
	}

	#[inline(always)]
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
		let mut entropy: [u8; std::mem::size_of::<u128>()] = Default::default();
		entropy.copy_from_slice(&crate::entropy::entropy_from_system(std::mem::size_of::<
			u128,
		>()));
		Self {
			seed: u128::from_ne_bytes(entropy),
			inc: 0,
			state: 0,
		}
	}
}

impl RNG for Pcg64 {
	type Output = [u8; 8];

	fn rand(&mut self) -> Self::Output {
		let ret = self.rand128();
		self.seed = self.state ^ (ret as u128).rotate_right(64);
		ret.to_ne_bytes()
	}

	fn rand_with_seed(_seed: &[u8]) -> Self::Output {
		unimplemented!("Pcg64 requires a state!");
	}

	fn reseed(&mut self, new_seed: &[u8]) {
		let mut seed = [0u8; 16];
		seed.iter_mut().zip(new_seed).for_each(|(a, b)| *a = *b);
		self.seed = u128::from_ne_bytes(seed);
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

#[cfg(feature = "std")]
impl std::fmt::Display for Pcg64 {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Pcg64 ({:p})", self)
	}
}
