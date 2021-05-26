#![no_main]
use libfuzzer_sys::fuzz_target;
use nanorand::{tls_rng, Rng};

fuzz_target!(|data: (u8, u8, bool)| {
	let (a, b, range_type) = data;
	let (lower, upper) = if a > b {
		(b as u64, a as u64)
	} else {
		(a as u64, b as u64)
	};
	if range_type {
		let number = tls_rng().generate_range(lower..=upper);
		if upper == lower {
			assert_eq!(number, upper);
		} else {
			assert!(number >= lower);
			assert!(number <= upper);
		}
	} else {
		let number = tls_rng().generate_range(lower..upper);
		if upper == lower {
			assert_eq!(number, upper);
		} else {
			assert!(number >= lower);
			assert!(number < upper);
		}
	};
});
