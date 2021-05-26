use nanorand::Rng;
use pyo3::prelude::*;

#[pyclass]
#[derive(Default)]
pub struct WyRand {
	inner: nanorand::WyRand,
}

#[pymethods]
impl WyRand {
	#[new]
	pub fn new() -> Self {
		WyRand {
			inner: nanorand::WyRand::new(),
		}
	}

	#[staticmethod]
	pub fn new_seed(seed: u64) -> Self {
		WyRand {
			inner: nanorand::WyRand::new_seed(seed),
		}
	}

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

	/// Generate a random boolean value from the provided RNG
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
}
