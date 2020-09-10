extern "C" {
	fn rand() -> u32;
}

pub fn entropy_from_system() -> u64 {
	let a = unsafe { rand() };
	let b = unsafe { rand() };
	(((a as u64) << 32) | (b as u64)).wrapping_mul(42)
}
