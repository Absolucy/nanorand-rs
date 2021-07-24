use crate::{Rng, WyRand};
use std::{cell::RefCell, rc::Rc};

thread_local! {
	static WYRAND: Rc<RefCell<WyRand>> = Rc::new(RefCell::new(WyRand::new()));
}

#[derive(Clone)]
#[doc(hidden)]
pub struct TlsWyRand(Rc<RefCell<WyRand>>);

impl Rng for TlsWyRand {
	type Output = [u8; 8];

	fn rand(&mut self) -> Self::Output {
		self.0.borrow_mut().rand()
	}

	fn rand_with_seed(seed: &[u8]) -> Self::Output {
		WyRand::rand_with_seed(seed)
	}

	fn reseed(&mut self, new_seed: &[u8]) {
		self.0.borrow_mut().reseed(new_seed)
	}
}

/// Fetch a thread-local [`WyRand`]
/// ```rust
/// use nanorand::Rng;
///
/// let mut rng = nanorand::tls_rng();
/// println!("Random number: {}", rng.generate::<u64>());
/// ```
/// This cannot be passed to another thread, as something like this will fail to compile:
/// ```compile_fail
/// use nanorand::Rng;
///
/// let mut rng = nanorand::tls_rng();
/// std::thread::spawn(move || {
///     println!("Random number: {}", rng.generate::<u64>());
/// });
/// ```
pub fn tls_rng() -> TlsWyRand {
	WYRAND.with(|tls| TlsWyRand(tls.clone()))
}
