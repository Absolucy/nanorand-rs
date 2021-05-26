#![no_main]
use libfuzzer_sys::fuzz_target;
use nanorand::{tls_rng, Rng};

fuzz_target!(|data: (u8, u8)| {
	let (a, b) = data;
	let (lower, upper) = if a > b { (b, a) } else { (a, b) };
	let number = tls_rng().generate_range(lower, upper);
	assert!(number >= lower);
	assert!(number <= upper);
});
