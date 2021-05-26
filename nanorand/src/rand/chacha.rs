use crate::{crypto::chacha, Rng};
use core::fmt::{self, Display, Formatter};
#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// An instance of the ChaCha random number generator.
/// Seeded from the system entropy generator when available.
/// **This generator _is theoretically_ cryptographically secure.**
#[cfg_attr(feature = "zeroize", derive(Zeroize))]
#[cfg_attr(feature = "zeroize", zeroize(drop))]
#[repr(C)]
pub struct ChaCha {
	state: [u32; 16],
	rounds: u8,
}

impl ChaCha {
	/// Create a new [`ChaCha`] instance, seeding from the system's default source of entropy.
	#[must_use]
	pub fn new(rounds: u8) -> Self {
		let mut key: [u8; 32] = Default::default();
		crate::entropy::system(&mut key);
		let mut nonce: [u8; 8] = Default::default();
		crate::entropy::system(&mut nonce);
		let state = chacha::chacha_init(key, nonce);
		Self { state, rounds }
	}

	/// Create a new [`ChaCha`] instance, using the provided key and nonce.
	#[must_use]
	pub const fn new_key(rounds: u8, key: [u8; 32], nonce: [u8; 8]) -> Self {
		let state = chacha::chacha_init(key, nonce);
		Self { state, rounds }
	}
}

impl Default for ChaCha {
	fn default() -> Self {
		let mut key: [u8; 32] = Default::default();
		crate::entropy::system(&mut key);
		let mut nonce: [u8; 8] = Default::default();
		crate::entropy::system(&mut nonce);
		let state = chacha::chacha_init(key, nonce);
		Self { state, rounds: 20 }
	}
}

impl Rng for ChaCha {
	type Output = [u8; 64];

	fn rand(&mut self) -> Self::Output {
		let block = chacha::chacha_block(self.rounds, self.state);
		let mut ret = [0_u8; 64];
		block.iter().enumerate().for_each(|(idx, num)| {
			let x = num.to_ne_bytes();
			let n = idx * 4;
			ret[n] = x[0];
			ret[n + 1] = x[1];
			ret[n + 2] = x[2];
			ret[n + 3] = x[3];
		});
		// Now, we're going to just increment our counter so we get an entirely new output next time.
		// If the counter overflows, we just reseed entirely instead.
		if !chacha::chacha_increment_counter(&mut self.state) {
			let mut new_seed: [u8; 40] = [42_u8; 40];
			crate::entropy::system(&mut new_seed);
			self.reseed(&new_seed);
		}
		ret
	}

	fn rand_with_seed(_seed: &[u8]) -> Self::Output {
		panic!("ChaCha RNG requires a state!");
	}

	fn reseed(&mut self, new_seed: &[u8]) {
		let mut seed = [42_u8; 40];
		seed.iter_mut().zip(new_seed).for_each(|(a, b)| *a = *b);
		let mut key = [0_u8; 32];
		let mut nonce = [0_u8; 8];
		key.copy_from_slice(&seed[..32]);
		nonce.copy_from_slice(&seed[32..40]);
		self.state = chacha::chacha_init(key, nonce);
	}
}

impl Clone for ChaCha {
	fn clone(&self) -> Self {
		Self {
			state: self.state,
			rounds: self.rounds,
		}
	}
}

impl Display for ChaCha {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "ChaCha ({:p}, {} rounds)", self, self.rounds)
	}
}
