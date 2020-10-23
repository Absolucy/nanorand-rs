use alloc::vec::Vec;

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
pub fn entropy_from_system(amt: usize) -> Vec<u8> {
	let mut entropy: Vec<u8> = vec![42; amt];
	match getrandom::getrandom(&mut entropy) {
		Ok(_) => entropy,
		Err(_) => backup_entropy(amt),
	}
}

/// Pull in backup entropy (rdseed and system time).
#[cfg(not(any(feature = "getrandom", unix, windows)))]
pub fn entropy_from_system(amt: usize) -> Vec<u8> {
	backup_entropy(amt)
}

#[cfg(feature = "std")]
/// An emergency system time-based entropy source.  
/// Should be slightly better than just piping the system time into a seed,
/// but for the love of god, don't use this unless you have a REALLY good reason.
pub fn emergency_system_time_entropy(amt: usize) -> Vec<u8> {
	use std::time::{SystemTime, UNIX_EPOCH};
	let mut entropy = vec![
		42_u8;
		((amt as f32 / core::mem::size_of::<u128>() as f32).ceil()
			* core::mem::size_of::<u128>() as f32)
			.round() as usize
	];
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
			.for_each(|(i, val)| entropy[(core::mem::size_of::<u128>() * n) + i] = *val);
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
			entropy[(core::mem::size_of::<u128>() * n) + i] =
				*val ^ entropy[(core::mem::size_of::<u128>() * n) + i]
		});
	}
	entropy[..amt].to_vec()
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
/// Returns [`Ok`] if it successfully managed to pull the requested amount of bytes.  
/// Returns [`Err`] if it couldn't pull the requested amount of bytes.  
/// ***VERY unreliable.***  
pub fn rdseed_entropy(amt: usize) -> Option<Result<Vec<u8>, Vec<u8>>> {
	if !std::is_x86_feature_detected!("rdseed") {
		return None;
	}

	let mut entropy: Vec<u8> = vec![
		42;
		((amt as f32 / core::mem::size_of::<u64>() as f32).ceil()
			* core::mem::size_of::<u64>() as f32)
			.round() as usize
	];
	let rdseed_amt = ((amt + core::mem::size_of::<u64>() - 1) / core::mem::size_of::<u64>()).max(0);
	for n in 0..rdseed_amt {
		let seed = match stupid_rdseed_hack() {
			Some(s) => s,
			None => return Some(Err(entropy)),
		};
		let x = if n % 2 == 0 {
			seed.to_le_bytes()
		} else {
			seed.to_be_bytes()
		};
		x.iter()
			.enumerate()
			.for_each(|(i, val)| entropy[(core::mem::size_of::<u64>() * n) + i] = *val);
	}
	Some(Ok(entropy))
}

/// A wrapper function for non-x86(64) platforms that do not have rdseed.
#[cfg(any(
	not(feature = "rdseed"),
	not(any(target_arch = "x86", target_arch = "x86_64"))
))]
pub fn rdseed_entropy(_amt: usize) -> Option<Result<Vec<u8>, Vec<u8>>> {
	None
}

#[cfg(feature = "std")]
/// A backup entropy source, trying rdseed first,
/// and if it fails or does not complete, combining it with or
/// using system time-based entropy generation.
pub fn backup_entropy(amt: usize) -> Vec<u8> {
	let mut entropy = vec![42_u8; amt];

	match rdseed_entropy(amt) {
		Some(rdseed) => match rdseed {
			Ok(o) => return o,
			Err(x) => {
				let len = x.len();
				x.iter()
					.enumerate()
					.for_each(|(i, val)| entropy[i % len] ^= *val);
			}
		},
		None => {}
	};

	let time_entropy = emergency_system_time_entropy(amt);
	let time_len = time_entropy.len();
	time_entropy
		.iter()
		.enumerate()
		.for_each(|(i, val)| entropy[i % time_len] ^= *val);

	emergency_system_time_entropy(amt)
}

#[cfg(not(feature = "std"))]
/// This just panics.
pub fn backup_entropy(_amt: usize) -> Vec<u8> {
	panic!("Failed to source any entropy!")
}
