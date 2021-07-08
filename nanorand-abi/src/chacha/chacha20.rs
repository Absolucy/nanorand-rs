use nanorand::{ChaCha20 as ChaCha, Rng};

/// Create a ChaCha RNG, using 20 rounds.
#[no_mangle]
pub extern "C" fn new_chacha20() -> ChaCha {
	ChaCha::new()
}

/// Create a ChaCha RNG, using 20 rounds,
/// and the provided 256-bit key and 64-bit nonce
#[no_mangle]
pub extern "C" fn new_chacha20_key(key: [u8; 32], nonce: [u8; 8]) -> ChaCha {
	ChaCha::new_key(key, nonce)
}

/// Get the raw 512-bit output from the provided ChaCha20 RNG.
/// You need to free this yourself!
#[no_mangle]
pub extern "C" fn chacha20_next(rng: &mut ChaCha) -> *mut u8 {
	let mut out = rng.rand();
	out.as_mut_ptr()
}

/// Generate a random 8-bit unsigned integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_u8(rng: &mut ChaCha) -> u8 {
	rng.generate()
}

/// Generate a random 16-bit unsigned integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_u16(rng: &mut ChaCha) -> u16 {
	rng.generate()
}

/// Generate a random 32-bit unsigned integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_u32(rng: &mut ChaCha) -> u32 {
	rng.generate()
}

/// Generate a random 64-bit unsigned integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_u64(rng: &mut ChaCha) -> u64 {
	rng.generate()
}

/// Generate a random pointer-sized unsigned integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_usize(rng: &mut ChaCha) -> usize {
	rng.generate()
}

/// Generate a random 8-bit signed integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_i8(rng: &mut ChaCha) -> i8 {
	rng.generate()
}

/// Generate a random 16-bit signed integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_i16(rng: &mut ChaCha) -> i16 {
	rng.generate()
}

/// Generate a random 32-bit signed integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_i32(rng: &mut ChaCha) -> i32 {
	rng.generate()
}

/// Generate a random 64-bit signed integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_i64(rng: &mut ChaCha) -> i64 {
	rng.generate()
}

/// Generate a random pointer-sized signed integer from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_isize(rng: &mut ChaCha) -> isize {
	rng.generate()
}

/// Generate a random boolean value from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_next_bool(rng: &mut ChaCha) -> bool {
	rng.generate()
}

/// Generate a random 8-bit unsigned integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_u8(rng: &mut ChaCha, lower: u8, upper: u8) -> u8 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 16-bit unsigned integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_u16(rng: &mut ChaCha, lower: u16, upper: u16) -> u16 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 32-bit unsigned integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_u32(rng: &mut ChaCha, lower: u32, upper: u32) -> u32 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 64-bit unsigned integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_u64(rng: &mut ChaCha, lower: u64, upper: u64) -> u64 {
	rng.generate_range(lower..=upper)
}

/// Generate a random pointer-sized unsigned integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_usize(rng: &mut ChaCha, lower: usize, upper: usize) -> usize {
	rng.generate_range(lower..=upper)
}

/// Generate a random 8-bit signed integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_i8(rng: &mut ChaCha, lower: i8, upper: i8) -> i8 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 16-bit signed integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_i16(rng: &mut ChaCha, lower: i16, upper: i16) -> i16 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 32-bit signed integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_i32(rng: &mut ChaCha, lower: i32, upper: i32) -> i32 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 64-bit signed integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_i64(rng: &mut ChaCha, lower: i64, upper: i64) -> i64 {
	rng.generate_range(lower..=upper)
}

/// Generate a random pointer-sized signed integer within a specified range from the provided ChaCha20 RNG
#[no_mangle]
pub extern "C" fn chacha20_range_isize(rng: &mut ChaCha, lower: isize, upper: isize) -> isize {
	rng.generate_range(lower..=upper)
}
