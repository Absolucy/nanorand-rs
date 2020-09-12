use super::emergency_system_time_entropy;
use core::ffi::c_void;
use core::ptr;

const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;

extern "system" {
	fn BCryptGenRandom(
		hAlgorithm: *mut c_void,
		pBuffer: *mut u8,
		cbBuffer: usize,
		dwFlags: u32,
	) -> u32;
}

/// Obtain a random 64-bit number using WinAPI's `BCryptGenRandom` function.
pub fn entropy_from_system(amt: usize) -> Vec<u8> {
	let mut entropy: Vec<u8> = vec![42; amt];
	let status: u32 = unsafe {
		BCryptGenRandom(
			ptr::null_mut(),
			entropy.as_mut_ptr(),
			amt,
			BCRYPT_USE_SYSTEM_PREFERRED_RNG,
		)
	};
	if status == 0 {
		entropy
	} else {
		emergency_system_time_entropy(amt)
	}
}
