/// An `no-std` compatible entropy generator for Unix-like platforms, using libc's `rand` function.
#[cfg(all(feature = "no_std", unix))]
#[allow(unsafe_code)]
pub mod unix_libc;
#[cfg(all(feature = "no_std", unix))]
pub use unix_libc as unix;
#[cfg(all(feature = "no_std", unix))]
pub use unix_libc::entropy_from_system;

/// A 100% safe entropy generator, using (in order of priority) `/dev/urandom`,
/// `/dev/random`, or the system time.
#[cfg(all(not(feature = "no_std"), unix))]
pub mod unix;
#[cfg(all(not(feature = "no_std"), unix))]
pub use unix::entropy_from_system;

/// An `no-std` compatible entropy generator for Windows, using WinAPI's `BCryptGenRandom` function.
#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::entropy_from_system;
