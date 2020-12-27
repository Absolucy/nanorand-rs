use nanorand::RNG;
use pyo3::prelude::*;

#[pyclass]
#[derive(Default)]
pub struct Pcg64 {
	inner: nanorand::Pcg64,
}

#[pymethods]
impl Pcg64 {
	#[new]
	pub fn new() -> Self {
		Pcg64 {
			inner: nanorand::Pcg64::new(),
		}
	}

	#[staticmethod]
	pub fn new_seed(seed: u128) -> Self {
		Pcg64 {
			inner: nanorand::Pcg64::new_seed(seed),
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
