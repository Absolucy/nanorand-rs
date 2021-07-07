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

			pub fn next_bytes(&mut self) -> Vec<u8> {
				self.inner.rand().to_vec()
			}

			pub fn next_u8(&mut self) -> u8 {
				self.inner.generate()
			}

			pub fn next_u16(&mut self) -> u16 {
				self.inner.generate()
			}

			pub fn next_u32(&mut self) -> u32 {
				self.inner.generate()
			}

			pub fn next_u64(&mut self) -> u64 {
				self.inner.generate()
			}

			pub fn next_usize(&mut self) -> usize {
				self.inner.generate()
			}

			pub fn next_bool(&mut self) -> bool {
				self.inner.generate()
			}

			pub fn range_u8(&mut self, lower: u8, upper: u8) -> u8 {
				self.inner.generate_range(lower..=upper)
			}

			pub fn range_u16(&mut self, lower: u16, upper: u16) -> u16 {
				self.inner.generate_range(lower..=upper)
			}

			pub fn range_u32(&mut self, lower: u32, upper: u32) -> u32 {
				self.inner.generate_range(lower..=upper)
			}

			pub fn range_u64(&mut self, lower: u64, upper: u64) -> u64 {
				self.inner.generate_range(lower..=upper)
			}
		}
	};
}

chacha_impl!(ChaCha8, nanorand::ChaCha8);
chacha_impl!(ChaCha12, nanorand::ChaCha12);
chacha_impl!(ChaCha20, nanorand::ChaCha20);
