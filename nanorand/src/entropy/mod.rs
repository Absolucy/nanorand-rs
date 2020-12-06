#[cfg(all(unix, not(feature = "getrandom")))]
pub use unix::entropy_from_system;
#[cfg(all(windows, not(target_vendor = "uwp"), not(feature = "getrandom")))]
pub use windows::entropy_from_system;
#[cfg(all(windows, target_vendor = "uwp", not(feature = "getrandom")))]
pub use windows_uwp::entropy_from_system;

#[cfg(all(unix, not(feature = "getrandom")))]
/// A 100% safe entropy generator, using (in order of priority) `/dev/urandom`,
/// `/dev/random`, or the system time.
pub mod unix;

#[cfg(all(windows, target_vendor = "uwp", not(feature = "getrandom")))]
/// An entropy generator for Windows, using WinAPI's `BCryptGenRandom` function.
pub mod windows_uwp;

#[cfg(all(windows, not(target_vendor = "uwp"), not(feature = "getrandom")))]
/// An entropy generator for Windows, using WinAPI's `RtlGenRandom` function.
pub mod windows;

#[cfg(feature = "getrandom")]
/// Pull in system entropy using the [`getrandom`](https://crates.io/crates/getrandom) crate.  
/// Uses backup entropy (rdseed and system time) if it fails.
pub fn entropy_from_system(out: &mut [u8]) {
	match getrandom::getrandom(out) {
		Ok(_) => (),
		Err(_) => backup_entropy(out),
	}
}

/// Pull in backup entropy (rdseed and system time).
#[cfg(not(any(feature = "getrandom", unix, windows)))]
pub fn entropy_from_system(out: &mut [u8]) -> Vec<u8> {
	backup_entropy(out)
}

#[cfg(feature = "std")]
/// An emergency system time-based entropy source.  
/// Should be slightly better than just piping the system time into a seed,
/// but for the love of god, don't use this unless you have a REALLY good reason.
pub fn emergency_system_time_entropy(out: &mut [u8]) {
	use std::time::{SystemTime, UNIX_EPOCH};
	let amt = out.len();

	let time_amt = ((amt + core::mem::size_of::<u128>() - 1) / core::mem::size_of::<u128>()).max(0);
	for n in 0..time_amt {
		let time = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		let x = if n % 2 == 0 {
			time.to_le_bytes()
		} else {
			time.to_be_bytes()
		};
		x.iter()
			.enumerate()
			.for_each(|(i, val)| out[(core::mem::size_of::<u128>() * n) + i] = *val);
	}
	for n in 0..time_amt {
		let time = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		let x = if n % 2 == 0 {
			time.to_be_bytes()
		} else {
			time.to_le_bytes()
		};
		x.iter().enumerate().for_each(|(i, val)| {
			out[(core::mem::size_of::<u128>() * n) + i] =
				*val ^ out[(core::mem::size_of::<u128>() * n) + i]
		});
	}
}

#[cfg(feature = "rdseed")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn stupid_rdseed_hack() -> Option<u64> {
	#[cfg(target_arch = "x86")]
	use core::arch::x86::_rdseed64_step as rdseed;
	#[cfg(target_arch = "x86_64")]
	use core::arch::x86_64::_rdseed64_step as rdseed;
	let mut x = 0;
	for _ in 0..10 {
		if 0 != unsafe { rdseed(&mut x) } {
			return Some(x);
		}
	}
	None
}

#[cfg(feature = "rdseed")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// An rdseed-based entropy source.  
/// Only works on x86/x86_64 platforms where the `rdseed` instructions are available.  
/// Returns [`None`] if `rdseed` is not available.  
/// Returns [`Some`] if it successfully managed to pull some bytes.
/// ***VERY unreliable.***  
pub fn rdseed_entropy(out: &mut [u8]) -> Option<usize> {
	if !std::is_x86_feature_detected!("rdseed") {
		return None;
	}
	let amt = out.len();
	let mut bytes_pulled: usize = 0;

	let rdseed_amt = ((amt + core::mem::size_of::<u64>() - 1) / core::mem::size_of::<u64>()).max(0);
	for n in 0..rdseed_amt {
		let seed = match stupid_rdseed_hack() {
			Some(s) => s,
			None => return Some(bytes_pulled),
		};
		let x = if n % 2 == 0 {
			seed.to_le_bytes()
		} else {
			seed.to_be_bytes()
		};
		bytes_pulled += x.len();
		x.iter()
			.enumerate()
			.for_each(|(i, val)| out[(core::mem::size_of::<u64>() * n) + i] = *val);
	}
	Some(bytes_pulled)
}

/// A wrapper function for non-x86(64) platforms that do not have rdseed.
#[cfg(any(
	not(feature = "rdseed"),
	not(any(target_arch = "x86", target_arch = "x86_64"))
))]
pub fn rdseed_entropy(_out: &mut [u8]) -> Option<usize> {
	None
}

#[cfg(feature = "std")]
/// A backup entropy source, trying rdseed first,
/// and if it fails or does not complete, combining it with or
/// using system time-based entropy generation.
pub fn backup_entropy(out: &mut [u8]) {
	if let Some(amt) = rdseed_entropy(out) {
		if amt >= out.len() {
			return;
		}
	};

	emergency_system_time_entropy(out);
}

#[cfg(not(feature = "std"))]
/// This just panics.
pub fn backup_entropy(out: &mut [u8]) {
	panic!("Failed to source any entropy!")
}
