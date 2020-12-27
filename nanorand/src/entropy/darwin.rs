use std::ffi::c_void;

#[link(name = "Security", kind = "framework")]
extern "C" {
	fn SecRandomCopyBytes(rnd: *const c_void, count: usize, bytes: *mut u8) -> u32;
}

/// Obtain a series of random bytes.
pub fn entropy_from_system(out: &mut [u8]) -> bool {
	unsafe { SecRandomCopyBytes(std::ptr::null(), out.len(), out.as_mut_ptr()) == 0 }
}
