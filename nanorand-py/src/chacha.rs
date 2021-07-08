use nanorand::Rng;
use pyo3::prelude::*;

macro_rules! chacha_impl {
	($type:ident, $nanorand_type:ty) => {
		#[pyclass]
		#[derive(Default)]
		pub struct $type {
			inner: $nanorand_type,
		}

		#[pymethods]
		impl $type {
			#[new]
			pub fn new() -> Self {
				Self {
					inner: <$nanorand_type>::new(),
				}
			}

			#[staticmethod]
			pub fn new_key(key: [u8; 32], nonce: [u8; 8]) -> Self {
				Self {
					inner: <$nanorand_type>::new_key(key, nonce),
				}
			}

			// Get the raw output of the provided RNG
			pub fn next_bytes(&mut self) -> Vec<u8> {
				self.inner.rand().to_vec()
			}

			/// Generate a random 8-bit unsigned integer from the provided RNG
			pub fn next_u8(&mut self) -> u8 {
				self.inner.generate()
			}

			/// Generate a random 16-bit unsigned integer from the provided RNG
			pub fn next_u16(&mut self) -> u16 {
				self.inner.generate()
			}

			/// Generate a random 32-bit unsigned integer from the provided RNG
			pub fn next_u32(&mut self) -> u32 {
				self.inner.generate()
			}

			/// Generate a random 64-bit unsigned integer from the provided RNG
			pub fn next_u64(&mut self) -> u64 {
				self.inner.generate()
			}

			/// Generate a random pointer-sized unsigned integer from the provided RNG
			pub fn next_usize(&mut self) -> usize {
				self.inner.generate()
			}

			/// Generate a random 8-bit signed integer from the provided RNG
			pub fn next_i8(&mut self) -> i8 {
				self.inner.generate()
			}

			/// Generate a random 16-bit signed integer from the provided RNG
			pub fn next_i16(&mut self) -> i16 {
				self.inner.generate()
			}

			/// Generate a random 32-bit signed integer from the provided RNG
			pub fn next_i32(&mut self) -> i32 {
				self.inner.generate()
			}

			/// Generate a random 64-bit signed integer from the provided RNG
			pub fn next_i64(&mut self) -> i64 {
				self.inner.generate()
			}

			/// Generate a random pointer-sized signed integer from the provided RNG
			pub fn next_isize(&mut self) -> isize {
				self.inner.generate()
			}

			/// Generate a random boolean from the provided RNG
			pub fn next_bool(&mut self) -> bool {
				self.inner.generate()
			}

			/// Generate a random 8-bit unsigned integer within a specified range from the provided RNG
			pub fn range_u8(&mut self, lower: u8, upper: u8) -> u8 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random 16-bit unsigned integer within a specified range from the provided RNG
			pub fn range_u16(&mut self, lower: u16, upper: u16) -> u16 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random 32-bit unsigned integer within a specified range from the provided RNG
			pub fn range_u32(&mut self, lower: u32, upper: u32) -> u32 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random 64-bit unsigned integer within a specified range from the provided RNG
			pub fn range_u64(&mut self, lower: u64, upper: u64) -> u64 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random pointer-sized unsigned integer within a specified range from the provided RNG
			pub fn range_usize(&mut self, lower: usize, upper: usize) -> usize {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random 8-bit signed integer within a specified range from the provided RNG
			pub fn range_i8(&mut self, lower: i8, upper: i8) -> i8 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random 16-bit signed integer within a specified range from the provided RNG
			pub fn range_i16(&mut self, lower: i16, upper: i16) -> i16 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random 32-bit signed integer within a specified range from the provided RNG
			pub fn range_i32(&mut self, lower: i32, upper: i32) -> i32 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random 64-bit signed integer within a specified range from the provided RNG
			pub fn range_i64(&mut self, lower: i64, upper: i64) -> i64 {
				self.inner.generate_range(lower..=upper)
			}

			/// Generate a random pointer-sized signed integer within a specified range from the provided RNG
			pub fn range_isize(&mut self, lower: isize, upper: isize) -> isize {
				self.inner.generate_range(lower..=upper)
			}
		}
	};
}

chacha_impl!(ChaCha8, nanorand::ChaCha8);
chacha_impl!(ChaCha12, nanorand::ChaCha12);
chacha_impl!(ChaCha20, nanorand::ChaCha20);
