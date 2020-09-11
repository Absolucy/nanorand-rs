/// Implementation of the wyrand RNG algorithm.  
/// More details can be seen at https://github.com/wangyi-fudan/wyhash
#[cfg(feature = "wyrand")]
pub mod wyrand;
#[cfg(feature = "wyrand")]
pub use wyrand::WyRand;

#[cfg(feature = "pcg64")]
pub mod pcg64;
#[cfg(feature = "pcg64")]
pub use pcg64::Pcg64;

use crate::gen::RandomGen;


/// A trait that represents a random number generator.
pub trait RNG: Clone {
	type Output: AsRef<[u8]>;

	/// Generates a random sequence of bytes, seeding from the internal state.
	fn rand(&mut self) -> Self::Output;
	/// Generates a random sequence of bytes, with a custom seed.
	fn rand_with_seed(seed: &[u8]) -> Self::Output;
	/// Generates a random of the specified type, seeding from the internal state.
	fn generate<R: RandomGen<Self>>(&mut self) -> R {
		R::generate(self)
	}
	/// Reseeds the RNG using a custom seed.
	fn reseed(&mut self, new_seed: &[u8]);
}
