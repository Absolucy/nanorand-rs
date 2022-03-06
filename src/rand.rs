#[cfg(feature = "chacha")]
pub use chacha::{ChaCha, ChaCha12, ChaCha20, ChaCha8};
#[cfg(feature = "pcg64")]
pub use pcg64::Pcg64;
#[cfg(feature = "wyrand")]
pub use wyrand::WyRand;

use crate::gen::{RandomGen, RandomRange};
use core::ops::RangeBounds;

/// Implementation of the wyrand PRNG algorithm.
/// More details can be seen at <https://github.com/wangyi-fudan/wyhash>
#[cfg(feature = "wyrand")]
pub mod wyrand;

/// Implementation of the Pcg64 PRNG algorithm.
/// More details can be seen at <https://www.pcg-random.org/index.html>
#[cfg(feature = "pcg64")]
pub mod pcg64;

/// Implementation of the ChaCha CSPRNG algorithm.
/// More details can be seen at <https://en.wikipedia.org/wiki/Salsa20>
#[cfg(feature = "chacha")]
pub mod chacha;

/// A trait that represents a random number generator.
pub trait Rng<const OUTPUT: usize>: Clone {
	/// Generates a random sequence of bytes, seeding from the internal state.
	fn rand(&mut self) -> [u8; OUTPUT];
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate<R>(&mut self) -> R
	where
		R: RandomGen<OUTPUT, Self>,
	{
		R::random(self)
	}
	/// Fill an array with the specified type.
	fn fill<R, A>(&mut self, mut target: A)
	where
		R: RandomGen<OUTPUT, Self>,
		A: AsMut<[R]>,
	{
		let target = target.as_mut();
		target.iter_mut().for_each(|entry| *entry = self.generate());
	}
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate_range<R, B>(&mut self, range: B) -> R
	where
		R: RandomRange<OUTPUT, Self>,
		B: RangeBounds<R>,
	{
		R::random_range(self, range)
	}
	/// Shuffle a slice, using the RNG.
	fn shuffle<I, S>(&mut self, mut target: S)
	where
		S: AsMut<[I]>,
	{
		let target = target.as_mut();
		let target_len = target.len();
		for idx in 0..target_len {
			let random_idx = self.generate_range(0..target_len);
			target.swap(idx, random_idx);
		}
	}
}

/// A trait that represents an RNG that can be reseeded from arbitrary bytes.
pub trait SeedableRng<const SEED_SIZE: usize, const OUTPUT: usize>: Rng<OUTPUT> {
	/// Re-seed the RNG with the specified bytes.
	fn reseed(&mut self, seed: [u8; SEED_SIZE]);
}
