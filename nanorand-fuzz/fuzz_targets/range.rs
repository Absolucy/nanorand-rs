#![no_main]
use libfuzzer_sys::fuzz_target;
use nanorand::{tls_rng, Rng};

fuzz_target!(|data: (u16, u16, bool)| {
	let (a, b, range_type) = data;
	let (lower, upper) = if a > b { (b, a) } else { (a, b) };
	if range_type {
		let number = tls_rng().generate_range(lower..=upper);
		if upper == lower {
			assert_eq!(
				number, upper,
				"{} was outside of range {}=={}",
				number, lower, upper
			);
		} else {
			assert!(
				number >= lower,
				"{} was bigger than range {}..={}",
				number,
				lower,
				upper
			);
			assert!(
				number <= upper,
				"{} was smaller than range {}..={}",
				number,
				lower,
				upper
			);
		}
	} else {
		let number = tls_rng().generate_range(lower..upper);
		if upper == lower {
			assert_eq!(
				number, upper,
				"{} was outside of range {}=={}",
				number, lower, upper
			);
		} else {
			assert!(
				number >= lower,
				"{} was bigger than range {}..{}",
				number,
				lower,
				upper
			);
			assert!(
				number < upper,
				"{} was smaller than range {}..{}",
				number,
				lower,
				upper
			);
		}
	};
});
