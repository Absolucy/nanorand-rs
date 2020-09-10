/// A 100% safe entropy generator, using (in order of priority) `/dev/urandom`,
/// `/dev/random`, or the system time.
#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
pub use unix::entropy_from_system;

/// An entropy generator for Windows, using WinAPI's `BCryptGenRandom` function.
#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::entropy_from_system;
