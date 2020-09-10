use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

/// Obtain a random 64-bit number.  
/// Sources (in order of priority):  
///  1. `/dev/urandom`  
///  2. `/dev/random`  
///  3. The current system time, in nanoseconds since the Unix Epoch.
pub fn entropy_from_system() -> u64 {
	match File::open("/dev/urandom") {
		Ok(mut fd) => {
			let mut entropy: [u8; 8] = Default::default();
			match fd.read_exact(&mut entropy) {
				Ok(_) => u64::from_ne_bytes(entropy),
				Err(_) => (u64::from_ne_bytes(entropy)
					^ ((SystemTime::now()
						.duration_since(UNIX_EPOCH)
						.unwrap()
						.as_nanos() >> 64) as u64))
					.wrapping_mul(42),
			}
		}
		Err(_) => {
			// Ugh, let's try for /dev/random
			match File::open("/dev/random") {
				Ok(mut fd) => {
					let mut entropy: [u8; 8] = Default::default();
					match fd.read_exact(&mut entropy) {
						Ok(_) => u64::from_ne_bytes(entropy),
						Err(_) => (u64::from_ne_bytes(entropy)
							^ ((SystemTime::now()
								.duration_since(UNIX_EPOCH)
								.unwrap()
								.as_nanos() >> 64) as u64))
							.wrapping_mul(42),
					}
				}
				Err(_) => {
					// Fuck it, just use system time and hope for the best.
					(SystemTime::now()
						.duration_since(UNIX_EPOCH)
						.unwrap()
						.as_nanos() >> 64) as u64
				}
			}
		}
	}
}
