use std::ffi::c_void;

extern "C" {
	fn getrandom(buf: *mut c_void, buflen: usize, flags: u32) -> isize;
}

/// Obtain a series of random bytes.  
pub fn entropy_from_system(out: &mut [u8]) -> bool {
	unsafe { getrandom(out.as_mut_ptr() as *mut c_void, out.len(), 0x0001) >= 1 }
}
