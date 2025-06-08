use core::ffi::{c_char, c_ulong, c_void};

extern "system" {
	#[link_name = "SystemFunction036"]
	fn RtlGenRandom(pBuffer: *mut c_void, cbBuffer: c_ulong) -> c_char;
}

/// Obtain a random 64-bit number using WinAPI's `RtlGenRandom` function.
pub fn entropy(out: &mut [u8]) -> bool {
	unsafe { RtlGenRandom(out.as_mut_ptr() as *mut c_void, out.len() as u32) == 0 }
}
