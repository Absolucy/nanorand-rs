use super::backup_entropy;

extern "system" {
	#[link_name = "SystemFunction036"]
	fn RtlGenRandom(pBuffer: *mut u8, cbBuffer: usize) -> u32;
}

/// Obtain a random 64-bit number using WinAPI's `RtlGenRandom` function.
pub fn entropy_from_system(out: &mut [u8]) -> bool {
	unsafe { RtlGenRandom(out.as_mut_ptr(), out.len()) == 0 }
}
