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
pub trait Rng: Clone {
	/// The byte output that this RNG emits.
	type Output: AsRef<[u8]>;

	/// Generates a random sequence of bytes, seeding from the internal state.
	fn rand(&mut self) -> Self::Output;
	/// Generates a random sequence of bytes, with a custom seed.
	fn rand_with_seed(seed: &[u8]) -> Self::Output;
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate<R>(&mut self) -> R
	where
		R: RandomGen<Self>,
	{
		R::random(self)
	}
	/// Fill an array with the specified type.
	fn fill<R, A>(&mut self, mut target: A)
	where
		R: RandomGen<Self>,
		A: AsMut<[R]>,
	{
		let target = target.as_mut();
		target.iter_mut().for_each(|entry| *entry = self.generate());
	}
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate_range<R, B>(&mut self, range: B) -> R
	where
		R: RandomRange<Self>,
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
	/// Reseeds the RNG using a custom seed.
	fn reseed(&mut self, new_seed: &[u8]);
}
