#[cfg(feature = "chacha")]
pub use chacha::ChaCha;
#[cfg(feature = "pcg64")]
pub use pcg64::Pcg64;
#[cfg(feature = "wyrand")]
pub use wyrand::WyRand;

use crate::gen::{RandomGen, RandomRange};

/// Implementation of the wyrand PRNG algorithm.
/// More details can be seen at https://github.com/wangyi-fudan/wyhash
#[cfg(feature = "wyrand")]
pub mod wyrand;

/// Implementation of the Pcg64 PRNG algorithm.
/// More details can be seen at https://www.pcg-random.org/index.html
#[cfg(feature = "pcg64")]
pub mod pcg64;

/// Implementation of the ChaCha CSPRNG algorithm.
/// More details can be seen at https://en.wikipedia.org/wiki/Salsa20
#[cfg(feature = "chacha")]
pub mod chacha;

/// A trait that represents a random number generator.
pub trait RNG: Clone {
	/// The byte output that this RNG emits.
	type Output: AsRef<[u8]>;

	/// Generates a random sequence of bytes, seeding from the internal state.
	fn rand(&mut self) -> Self::Output;
	/// Generates a random sequence of bytes, with a custom seed.
	fn rand_with_seed(seed: &[u8]) -> Self::Output;
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate<R: RandomGen<Self>>(&mut self) -> R {
		R::random(self)
	}
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate_range<R: RandomRange<Self>>(&mut self, lower: R, upper: R) -> R {
		R::random_range(self, lower, upper)
	}
	/// Shuffle a slice, using the RNG.
	fn shuffle<I, S: AsMut<[I]>>(&mut self, mut target: S) {
		let target = target.as_mut();
		let target_len = target.len();
		for idx in 0..target_len {
			let random_idx = self.generate_range::<usize>(0, target_len - 1);
			target.swap(idx, random_idx);
		}
	}
	/// Reseeds the RNG using a custom seed.
	fn reseed(&mut self, new_seed: &[u8]);
}
