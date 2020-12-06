use nanorand::RNG;
use pyo3::prelude::*;

#[pyclass]
#[derive(Default)]
pub struct ChaCha {
	inner: nanorand::ChaCha,
}

#[pymethods]
impl ChaCha {
	#[new]
	pub fn new(rounds: u8) -> Self {
		Self {
			inner: nanorand::ChaCha::new(rounds),
		}
	}

	#[staticmethod]
	pub fn new_key(rounds: u8, key: [u8; 32], nonce: [u8; 8]) -> Self {
		Self {
			inner: nanorand::ChaCha::new_key(rounds, key, nonce),
		}
	}

	#[staticmethod]
	pub fn new_chacha8() -> Self {
		Self {
			inner: nanorand::ChaCha::new(8),
		}
	}

	#[staticmethod]
	pub fn new_chacha8_key(key: [u8; 32], nonce: [u8; 8]) -> Self {
		Self {
			inner: nanorand::ChaCha::new_key(8, key, nonce),
		}
	}

	#[staticmethod]
	pub fn new_chacha12() -> Self {
		Self {
			inner: nanorand::ChaCha::new(12),
		}
	}

	#[staticmethod]
	pub fn new_chacha12_key(key: [u8; 32], nonce: [u8; 8]) -> Self {
		Self {
			inner: nanorand::ChaCha::new_key(12, key, nonce),
		}
	}

	#[staticmethod]
	pub fn new_chacha20() -> Self {
		Self {
			inner: nanorand::ChaCha::new(20),
		}
	}

	#[staticmethod]
	pub fn new_chacha20_key(key: [u8; 32], nonce: [u8; 8]) -> Self {
		Self {
			inner: nanorand::ChaCha::new_key(20, key, nonce),
		}
	}

	pub fn next(&mut self) -> Vec<u8> {
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
		self.inner.generate_range(lower, upper)
	}

	pub fn range_u16(&mut self, lower: u16, upper: u16) -> u16 {
		self.inner.generate_range(lower, upper)
	}

	pub fn range_u32(&mut self, lower: u32, upper: u32) -> u32 {
		self.inner.generate_range(lower, upper)
	}

	pub fn range_u64(&mut self, lower: u64, upper: u64) -> u64 {
		self.inner.generate_range(lower, upper)
	}
}
