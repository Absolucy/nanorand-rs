use nanorand::{Pcg64, Rng};

/// Create a new Pcg64 RNG, using system-provided entropy.
#[no_mangle]
pub extern "C" fn new_pcg64() -> Pcg64 {
	Pcg64::new()
}

/// Create a new Pcg64 RNG, using a given seed.
#[no_mangle]
pub extern "C" fn new_pcg64_seed(seed: [u8; 16]) -> Pcg64 {
	Pcg64::new_seed(u128::from_ne_bytes(seed))
}

/// Get the raw 64-bit output from the provided RNG.
/// You need to free this yourself!
#[no_mangle]
pub extern "C" fn pcg64_next(rng: &mut Pcg64) -> *mut u8 {
	let mut out = rng.rand();
	out.as_mut_ptr()
}

/// Generate a random 8-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_u8(rng: &mut Pcg64) -> u8 {
	rng.generate()
}

/// Generate a random 16-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_u16(rng: &mut Pcg64) -> u16 {
	rng.generate()
}

/// Generate a random 32-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_u32(rng: &mut Pcg64) -> u32 {
	rng.generate()
}

/// Generate a random 64-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_u64(rng: &mut Pcg64) -> u64 {
	rng.generate()
}

/// Generate a random pointer-sized unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_usize(rng: &mut Pcg64) -> usize {
	rng.generate()
}

/// Generate a random 8-bit signed integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_i8(rng: &mut Pcg64) -> i8 {
	rng.generate()
}

/// Generate a random 16-bit signed integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_i16(rng: &mut Pcg64) -> i16 {
	rng.generate()
}

/// Generate a random 32-bit signed integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_i32(rng: &mut Pcg64) -> i32 {
	rng.generate()
}

/// Generate a random 64-bit signed integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_i64(rng: &mut Pcg64) -> i64 {
	rng.generate()
}

/// Generate a random pointer-sized signed integer from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_isize(rng: &mut Pcg64) -> isize {
	rng.generate()
}

/// Generate a random boolean value from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_next_bool(rng: &mut Pcg64) -> bool {
	rng.generate()
}

/// Generate a random 8-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_u8(rng: &mut Pcg64, lower: u8, upper: u8) -> u8 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 16-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_u16(rng: &mut Pcg64, lower: u16, upper: u16) -> u16 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 32-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_u32(rng: &mut Pcg64, lower: u32, upper: u32) -> u32 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 64-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_u64(rng: &mut Pcg64, lower: u64, upper: u64) -> u64 {
	rng.generate_range(lower..=upper)
}

/// Generate a random pointer-sized unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_usize(rng: &mut Pcg64, lower: usize, upper: usize) -> usize {
	rng.generate_range(lower..=upper)
}

/// Generate a random 8-bit signed integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_i8(rng: &mut Pcg64, lower: i8, upper: i8) -> i8 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 16-bit signed integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_i16(rng: &mut Pcg64, lower: i16, upper: i16) -> i16 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 32-bit signed integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_i32(rng: &mut Pcg64, lower: i32, upper: i32) -> i32 {
	rng.generate_range(lower..=upper)
}

/// Generate a random 64-bit signed integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_i64(rng: &mut Pcg64, lower: i64, upper: i64) -> i64 {
	rng.generate_range(lower..=upper)
}

/// Generate a random pointer-sized signed integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn pcg64_range_isize(rng: &mut Pcg64, lower: isize, upper: isize) -> isize {
	rng.generate_range(lower..=upper)
}
