use core::ffi::c_void;
use core::ptr;

const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;

extern "system" {
	fn BCryptGenRandom(
		hAlgorithm: *mut c_void,
		pBuffer: *mut u8,
		cbBuffer: u32,
		dwFlags: u32,
	) -> u32;
}

/// Obtain a random 64-bit number using WinAPI's `BCryptGenRandom` function.
pub fn entropy_from_system() -> u64 {
	let mut entropy: [u8; 8] = Default::default();
	unsafe {
		BCryptGenRandom(
			ptr::null_mut(),
			entropy.as_mut_ptr(),
			8,
			BCRYPT_USE_SYSTEM_PREFERRED_RNG,
		)
	};
	u64::from_ne_bytes(entropy)
}
