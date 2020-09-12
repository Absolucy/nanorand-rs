/// A 100% safe entropy generator, using (in order of priority) `/dev/urandom`,
/// `/dev/random`, or the system time.
#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
pub use unix::entropy_from_system;

/// An entropy generator for Windows, using WinAPI's `BCryptGenRandom` function.
#[cfg(windows)]
#[allow(unsafe_code)]
pub mod windows;
#[cfg(windows)]
pub use windows::entropy_from_system;

use std::time::{SystemTime, UNIX_EPOCH};

/// An emergency system time-based entropy source.  
/// Should be slightly better than just piping the system time into a seed,
/// but for the love of god, don't use this unless you have a REALLY good reason.
#[cfg(feature = "std")]
pub fn emergency_system_time_entropy(amt: usize) -> Vec<u8> {
	let mut entropy: Vec<u8> = vec![
		42;
		((amt as f32 / std::mem::size_of::<u128>() as f32).ceil()
			* std::mem::size_of::<u128>() as f32)
			.round() as usize
	];
	let time_amt = ((amt + std::mem::size_of::<u128>() - 1) / std::mem::size_of::<u128>()).max(0);
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
			.for_each(|(i, val)| entropy[(std::mem::size_of::<u128>() * n) + i] = *val);
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
			entropy[(std::mem::size_of::<u128>() * n) + i] =
				*val ^ entropy[(std::mem::size_of::<u128>() * n) + i]
		});
	}
	entropy[..amt].to_vec()
}
