// Based off lemire's wyrand C++ code at https://github.com/lemire/testingRNG/blob/master/source/wyrand.h

use crate::RNG_STATE_GLOBAL;
use core::sync::atomic::Ordering;

/// Generate a random number with wyrand using a custom seed.
pub fn rand(state: u64) -> u64 {
	let state = state.wrapping_add(0xa0761d6478bd642f);
	let t: u128 = (state as u128).wrapping_mul((state ^ 0xe7037ed1a0b428db) as u128);
	((t >> 64) ^ t) as u64
}

/// Generate a random number with wyrand using nanorand's global state.
pub fn rand_global() -> u64 {
	let state = RNG_STATE_GLOBAL.load(Ordering::Relaxed);
	let result = rand(state);
	RNG_STATE_GLOBAL.store(result, Ordering::Relaxed);
	result
}
