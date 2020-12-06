use nanorand::RNG;

#[repr(transparent)]
pub struct Pcg64(nanorand::Pcg64);

impl Pcg64 {
	#[no_mangle]
	pub extern "C" fn new_pcg64() -> Self {
		Pcg64(nanorand::Pcg64::new())
	}

	#[no_mangle]
	pub extern "C" fn new_pcg64_seed(seed: u128) -> Self {
		Pcg64(nanorand::Pcg64::new_seed(seed))
	}

	#[no_mangle]
	pub extern "C" fn pcg64_next(rng: &mut Self) -> [u8; 8] {
		rng.0.rand()
	}

	#[no_mangle]
	pub extern "C" fn pcg64_next_u8(rng: &mut Self) -> u8 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn pcg64_next_u16(rng: &mut Self) -> u16 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn pcg64_next_u32(rng: &mut Self) -> u32 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn pcg64_next_u64(rng: &mut Self) -> u64 {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn pcg64_next_bool(rng: &mut Self) -> bool {
		rng.0.generate()
	}

	#[no_mangle]
	pub extern "C" fn pcg64_range_u8(rng: &mut Self, lower: u8, upper: u8) -> u8 {
		rng.0.generate_range(lower, upper)
	}

	#[no_mangle]
	pub extern "C" fn pcg64_range_u16(rng: &mut Self, lower: u16, upper: u16) -> u16 {
		rng.0.generate_range(lower, upper)
	}

	#[no_mangle]
	pub extern "C" fn pcg64_range_u32(rng: &mut Self, lower: u32, upper: u32) -> u32 {
		rng.0.generate_range(lower, upper)
	}

	#[no_mangle]
	pub extern "C" fn pcg64_range_u64(rng: &mut Self, lower: u64, upper: u64) -> u64 {
		rng.0.generate_range(lower, upper)
	}
}
