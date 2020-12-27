use nanorand::{ChaCha, RNG};

/// Create a ChaCha RNG, using the specified number of rounds
#[no_mangle]
pub extern "C" fn new_chacha(rounds: u8) -> ChaCha {
	ChaCha::new(rounds)
}

/// Create a ChaCha RNG, using the specified number of rounds,
/// and the provided 256-bit key and 64-bit nonce
#[no_mangle]
pub extern "C" fn new_chacha_key(rounds: u8, key: [u8; 32], nonce: [u8; 8]) -> ChaCha {
	ChaCha::new_key(rounds, key, nonce)
}

/// Create a ChaCha RNG using 8 rounds
#[no_mangle]
pub extern "C" fn new_chacha8() -> ChaCha {
	ChaCha::new(8)
}

/// Create a ChaCha RNG, using 8 rounds,
/// and the provided 256-bit key and 64-bit nonce
#[no_mangle]
pub extern "C" fn new_chacha8_key(key: [u8; 32], nonce: [u8; 8]) -> ChaCha {
	ChaCha::new_key(8, key, nonce)
}

/// Create a ChaCha RNG, using 12 rounds
#[no_mangle]
pub extern "C" fn new_chacha12() -> ChaCha {
	ChaCha::new(12)
}

/// Create a ChaCha RNG, using 12 rounds,
/// and the provided 256-bit key and 64-bit nonce
#[no_mangle]
pub extern "C" fn new_chacha12_key(key: [u8; 32], nonce: [u8; 8]) -> ChaCha {
	ChaCha::new_key(12, key, nonce)
}

/// Create a ChaCha RNG, using 20 rounds
#[no_mangle]
pub extern "C" fn new_chacha20() -> ChaCha {
	ChaCha::new(20)
}

/// Create a ChaCha RNG, using 20 rounds,
/// and the provided 256-bit key and 64-bit nonce
#[no_mangle]
pub extern "C" fn new_chacha20_key(key: [u8; 32], nonce: [u8; 8]) -> ChaCha {
	ChaCha::new_key(20, key, nonce)
}

/// Get the raw 512-bit output from the provided RNG.
/// You need to free this yourself!
#[no_mangle]
pub extern "C" fn chacha_next(rng: &mut ChaCha) -> *mut u8 {
	let mut out = rng.rand();
	out.as_mut_ptr()
}

/// Generate a random 8-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_next_u8(rng: &mut ChaCha) -> u8 {
	rng.generate()
}

/// Generate a random 16-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_next_u16(rng: &mut ChaCha) -> u16 {
	rng.generate()
}

/// Generate a random 32-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_next_u32(rng: &mut ChaCha) -> u32 {
	rng.generate()
}

/// Generate a random 64-bit unsigned integer from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_next_u64(rng: &mut ChaCha) -> u64 {
	rng.generate()
}

/// Generate a random boolean value from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_next_bool(rng: &mut ChaCha) -> bool {
	rng.generate()
}

/// Generate a random 8-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_range_u8(rng: &mut ChaCha, lower: u8, upper: u8) -> u8 {
	rng.generate_range(lower, upper)
}

/// Generate a random 16-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_range_u16(rng: &mut ChaCha, lower: u16, upper: u16) -> u16 {
	rng.generate_range(lower, upper)
}

/// Generate a random 32-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_range_u32(rng: &mut ChaCha, lower: u32, upper: u32) -> u32 {
	rng.generate_range(lower, upper)
}

/// Generate a random 64-bit unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_range_u64(rng: &mut ChaCha, lower: u64, upper: u64) -> u64 {
	rng.generate_range(lower, upper)
}

/// Generate a random pointer-sized unsigned integer within a specified range from the provided RNG
#[no_mangle]
pub extern "C" fn chacha_range_usize(rng: &mut ChaCha, lower: usize, upper: usize) -> usize {
	rng.generate_range(lower, upper)
}
