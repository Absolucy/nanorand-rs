use super::backup_entropy;
use std::fs::File;
use std::io::{BufReader, Read};

/// Obtain a series of random bytes.  
/// Sources (in order of priority):  
///  1. `/dev/urandom`  
///  2. `/dev/random`  
///  3. rdseed (if available)
///  4. The emergency time-based entropy source.
pub fn entropy_from_system(amt: usize) -> Vec<u8> {
	match File::open("/dev/urandom") {
		Ok(fd) => {
			let mut entropy: Vec<u8> = vec![42; amt];
			let mut reader = BufReader::new(fd);
			match reader.read_exact(&mut entropy[..amt]) {
				Ok(_) => entropy,
				Err(_) => backup_entropy(amt),
			}
		}
		Err(_) => {
			// Ugh, let's try for /dev/random
			match File::open("/dev/random") {
				Ok(fd) => {
					let mut entropy: Vec<u8> = vec![42; amt];
					let mut reader = BufReader::new(fd);
					match reader.read_exact(&mut entropy[..amt]) {
						Ok(_) => entropy,
						Err(_) => backup_entropy(amt),
					}
				}
				Err(_) => backup_entropy(amt),
			}
		}
	}
}
