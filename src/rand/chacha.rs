use super::RNG;
use crate::crypto::chacha;

/// An instance of the ChaCha random number generator.  
/// Seeded from the system entropy generator when available.  
/// **This generator _is theoretically_ cryptographically secure.**
pub struct ChaCha {
	key: [u8; 32],
	nonce: [u8; 16],
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
		let state = chacha::chacha_init(rounds, key, nonce);
		Self {
			key,
			nonce,
			rounds,
			state,
		}
	}

	/// Create a new [`ChaCha`] instance, using the provided key and nonce.
	pub fn new_key(rounds: u8, key: [u8; 32], nonce: [u8; 16]) -> Self {
		let state = chacha::chacha_init(rounds, key, nonce);
		Self {
			key,
			nonce,
			rounds,
			state,
		}
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
		let state = chacha::chacha_init(20, key, nonce);
		Self {
			key,
			nonce,
			state,
			rounds: 20,
		}
	}
}

impl RNG for ChaCha {
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

impl Clone for ChaCha {
	fn clone(&self) -> Self {
		Self { key: self.key, nonce: self.nonce, state: self.state, rounds: self.rounds }
	}
}
