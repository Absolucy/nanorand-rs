use nanorand::RNG;

#[repr(transparent)]
pub struct ChaCha(nanorand::ChaCha);

impl ChaCha {
	#[no_mangle]
	pub extern "C" fn new_chacha(rounds: u8) -> Self {
		ChaCha(nanorand::ChaCha::new(rounds))
	}

	#[no_mangle]
	pub extern "C" fn new_chacha_key(rounds: u8, key: [u8; 32], nonce: [u8; 8]) -> Self {
		ChaCha(nanorand::ChaCha::new_key(rounds, key, nonce))
	}

	#[no_mangle]
	pub extern "C" fn new_chacha8() -> Self {
		ChaCha(nanorand::ChaCha::new(8))
	}

	#[no_mangle]
	pub extern "C" fn new_chacha8_key(key: [u8; 32], nonce: [u8; 8]) -> Self {
		ChaCha(nanorand::ChaCha::new_key(8, key, nonce))
	}

	#[no_mangle]
	pub extern "C" fn new_chacha12() -> Self {
		ChaCha(nanorand::ChaCha::new(12))
	}

	#[no_mangle]
	pub extern "C" fn new_chacha12_key(key: [u8; 32], nonce: [u8; 8]) -> Self {
		ChaCha(nanorand::ChaCha::new_key(12, key, nonce))
	}

	#[no_mangle]
	pub extern "C" fn new_chacha20() -> Self {
		ChaCha(nanorand::ChaCha::new(20))
	}

	#[no_mangle]
	pub extern "C" fn new_chacha20_key(key: [u8; 32], nonce: [u8; 8]) -> Self {
		ChaCha(nanorand::ChaCha::new_key(20, key, nonce))
	}

	#[no_mangle]
	pub extern "C" fn chacha_next(rng: &mut Self) -> [u8; 64] {
		rng.0.rand()
	}

	#[no_mangle]
	pub extern "C" fn chacha_next_u8(rng: &mut Self) -> u8 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn chacha_next_u16(rng: &mut Self) -> u16 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn chacha_next_u32(rng: &mut Self) -> u32 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn chacha_next_u64(rng: &mut Self) -> u64 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn chacha_next_bool(rng: &mut Self) -> bool {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn chacha_range_u8(rng: &mut Self, lower: u8, upper: u8) -> u8 {
		rng.0.generate_range(lower, upper)
	}

	#[no_mangle]
	pub extern "C" fn chacha_range_u16(rng: &mut Self, lower: u16, upper: u16) -> u16 {
		rng.0.generate_range(lower, upper)
	}

	#[no_mangle]
	pub extern "C" fn chacha_range_u32(rng: &mut Self, lower: u32, upper: u32) -> u32 {
		rng.0.generate_range(lower, upper)
	}

	#[no_mangle]
	pub extern "C" fn chacha_range_u64(rng: &mut Self, lower: u64, upper: u64) -> u64 {
		rng.0.generate_range(lower, upper)
	}
}
