/// Implementation of the wyrand PRNG algorithm.  
/// More details can be seen at https://github.com/wangyi-fudan/wyhash
#[cfg(feature = "wyrand")]
pub mod wyrand;
#[cfg(feature = "wyrand")]
pub use wyrand::WyRand;

/// Implementation of the Pcg64 PRNG algorithm.  
/// More details can be seen at https://www.pcg-random.org/index.html
#[cfg(feature = "pcg64")]
pub mod pcg64;
#[cfg(feature = "pcg64")]
pub use pcg64::Pcg64;

/// Implementation of the ChaCha CSPRNG algorithm.  
/// More details can be seen at https://en.wikipedia.org/wiki/Salsa20
#[cfg(feature = "chacha")]
pub mod chacha;
#[cfg(feature = "chacha")]
pub use chacha::ChaCha;

use crate::gen::{RandomGen, RandomRange};

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
	/// We use one byte per comparison, creating a value between -1 and 1, by drawing two bits
	/// from a byte. The first bit determines signedness (negative or positive), the next determines
	/// 0 or 1. The comparison is then used based on this number.
	#[cfg(feature = "std")]
	fn shuffle<I, S: AsMut<[I]>>(&mut self, mut target: S) {
		use std::cmp::Ordering;
		let mut bytes: Vec<u8> = Vec::with_capacity(core::mem::size_of::<Self::Output>());
		let mut cursor = 0_usize;
		let target = target.as_mut();
		target.sort_by(|_a, _b| {
			if cursor >= bytes.len() {
				let rand_bytes = self.rand();
				bytes.extend_from_slice(rand_bytes.as_ref());
			}
			let our_byte = bytes[cursor];
			cursor += 1;
			let signedness = ((our_byte >> 1) & 1) == 0;
			let number = ((our_byte >> 7) & 1) as i8;
			let ordering_val = if signedness { -number } else { number };
			match ordering_val {
				0 => Ordering::Equal,
				1 => Ordering::Greater,
				_ => Ordering::Less,
			}
		});
	}
	/// Reseeds the RNG using a custom seed.
	fn reseed(&mut self, new_seed: &[u8]);
}
