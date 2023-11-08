use core::ffi::c_void;

extern "C" {
	fn arc4random_buf(buf: *mut c_void, nbytes: usize);
}

/// Obtain a series of random bytes.
pub fn entropy(out: &mut [u8]) -> bool {
	unsafe {
		arc4random_buf(out.as_mut_ptr() as *mut c_void, out.len());
		true
	}
}
