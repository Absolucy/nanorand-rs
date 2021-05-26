use nanorand::{Rng, WyRand};

/// Create a new Pcg64 RNG, using system-provided entropy.
#[no_mangle]
pub extern "C" fn new_wyrand() -> WyRand {
	WyRand::new()
}

/// Create a new Pcg64 RNG, using a given seed.
#[no_mangle]
pub extern "C" fn new_wyrand_with_seed(seed: u64) -> WyRand {
	WyRand::new_seed(seed)
}

/// Get the raw 64-bit output from the provided RNG.
/// You need to free this yourself!
#[no_mangle]
pub extern "C" fn wyrand_next(rng: &mut WyRand) -> *mut u8 {
	let mut out = rng.rand();
	out.as_mut_ptr()
}

/// Generate a random 8-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_next_u8(rng: &mut WyRand) -> u8 {
	rng.generate()
}

/// Generate a random 16-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_next_u16(rng: &mut WyRand) -> u16 {
	rng.generate()
}

/// Generate a random 32-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_next_u32(rng: &mut WyRand) -> u32 {
	rng.generate()
}

/// Generate a random 64-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_next_u64(rng: &mut WyRand) -> u64 {
	rng.generate()
}

/// Generate a random pointer-sized unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_next_usize(rng: &mut WyRand) -> usize {
	rng.generate()
}

/// Generate a random boolean value from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_next_bool(rng: &mut WyRand) -> bool {
	rng.generate()
}

/// Generate a random 8-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_range_u8(rng: &mut WyRand, lower: u8, upper: u8) -> u8 {
	rng.generate_range(lower, upper)
}

/// Generate a random 16-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_range_u16(rng: &mut WyRand, lower: u16, upper: u16) -> u16 {
	rng.generate_range(lower, upper)
}

/// Generate a random 32-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_range_u32(rng: &mut WyRand, lower: u32, upper: u32) -> u32 {
	rng.generate_range(lower, upper)
}

/// Generate a random 64-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_range_u64(rng: &mut WyRand, lower: u64, upper: u64) -> u64 {
	rng.generate_range(lower, upper)
}

/// Generate a random pointer-sized unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn wyrand_range_usize(rng: &mut WyRand, lower: usize, upper: usize) -> usize {
	rng.generate_range(lower, upper)
}
