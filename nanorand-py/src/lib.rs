pub mod chacha;
pub mod pcg64;
pub mod wyrand;

use pyo3::prelude::*;

#[pymodule]
fn pynanorand(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<chacha::ChaCha>()?;
	m.add_class::<pcg64::Pcg64>()?;
	m.add_class::<wyrand::WyRand>()?;

	Ok(())
}
