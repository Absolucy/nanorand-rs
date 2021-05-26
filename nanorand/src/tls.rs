use crate::WyRand;
use std::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
	rc::Rc,
};

thread_local! {
	static WYRAND: Rc<UnsafeCell<WyRand>> = Rc::new(UnsafeCell::new(WyRand::new()));
}

#[doc(hidden)]
pub struct TlsWyRand(Rc<UnsafeCell<WyRand>>);

impl Deref for TlsWyRand {
	type Target = WyRand;

	/// Safety: [`TlsWyRand`] is neither [Send] nor [Sync], and thus,
	/// there will always be a thread-local [`WyRand`] when there is a [`TlsWyRand`]
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0.get() }
	}
}

impl DerefMut for TlsWyRand {
	/// Safety: [`TlsWyRand`] is neither [Send] nor [Sync], and thus,
	/// there will always be a thread-local [`WyRand`] when there is a [`TlsWyRand`]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(*self.0).get() }
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
