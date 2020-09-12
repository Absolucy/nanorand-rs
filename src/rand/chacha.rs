use super::RNG;
use crate::crypto::chacha;

/// An instance of the ChaCha random number generator.  
/// Seeded from the system entropy generator when available.  
/// **This generator _is theoretically_ cryptographically secure.**
pub struct ChaCha {
	state: [u32; 16],
	rounds: u8,
}

impl ChaCha {
	/// Create a new [`ChaCha`] instance, seeding from the system's default source of entropy.
	#[cfg(feature = "std")]
	pub fn new(rounds: u8) -> Self {
		let mut key: [u8; std::mem::size_of::<u8>() * 32] = Default::default();
		key.copy_from_slice(&crate::entropy::entropy_from_system(
			std::mem::size_of::<u8>() * 32,
		));
		let mut nonce: [u8; std::mem::size_of::<u8>() * 16] = Default::default();
		nonce.copy_from_slice(&crate::entropy::entropy_from_system(
			std::mem::size_of::<u8>() * 16,
		));
		let state = chacha::chacha_init(key, nonce);
		Self { rounds, state }
	}

	/// Create a new [`ChaCha`] instance, using the provided key and nonce.
	pub fn new_key(rounds: u8, key: [u8; 32], nonce: [u8; 16]) -> Self {
		let state = chacha::chacha_init(key, nonce);
		Self { rounds, state }
	}
}

#[cfg(feature = "std")]
impl Default for ChaCha {
	fn default() -> Self {
		let mut key: [u8; std::mem::size_of::<u8>() * 32] = Default::default();
		key.copy_from_slice(&crate::entropy::entropy_from_system(
			std::mem::size_of::<u8>() * 32,
		));
		let mut nonce: [u8; std::mem::size_of::<u8>() * 16] = Default::default();
		nonce.copy_from_slice(&crate::entropy::entropy_from_system(
			std::mem::size_of::<u8>() * 16,
		));
		let state = chacha::chacha_init(key, nonce);
		Self { state, rounds: 20 }
	}
}

impl RNG for ChaCha {
	type Output = [u8; 64];

	fn rand(&mut self) -> Self::Output {
		let block = chacha::chacha_block(self.rounds, self.state);
		let mut ret = [0u8; 64];
		block.iter().enumerate().for_each(|(idx, num)| {
			let x = num.to_ne_bytes();
			let n = idx * 4;
			ret[n] = x[0];
			ret[n + 1] = x[1];
			ret[n + 2] = x[2];
			ret[n + 3] = x[3];
		});
		self.state[12] = self.state[12].wrapping_add(1);
		ret
	}

	fn rand_with_seed(_seed: &[u8]) -> Self::Output {
		panic!("ChaCha RNG requires a state!");
	}

	fn reseed(&mut self, new_seed: &[u8]) {
		let mut seed = [42u8; 48];
		seed.iter_mut().zip(new_seed).for_each(|(a, b)| *a = *b);
		let mut key = [0u8; 32];
		let mut nonce = [0u8; 16];
		key.copy_from_slice(&seed[..32]);
		nonce.copy_from_slice(&seed[32..48]);
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

#[cfg(feature = "std")]
impl std::fmt::Display for ChaCha {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "ChaCha ({:p}, {} rounds)", self, self.rounds)
	}
}
