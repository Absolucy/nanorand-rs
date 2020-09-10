#[cfg(all(feature = "no_std", unix))]
#[allow(unsafe_code)]
pub mod unix_libc;
#[cfg(all(feature = "no_std", unix))]
pub use unix_libc as unix;
#[cfg(all(feature = "no_std", unix))]
pub use unix_libc::entropy_from_system;

#[cfg(all(not(feature = "no_std"), unix))]
pub mod unix;
#[cfg(all(not(feature = "no_std"), unix))]
pub use unix::entropy_from_system;

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::entropy_from_system;
