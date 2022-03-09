#![deny(clippy::complexity, clippy::correctness, clippy::perf, clippy::style)]
#![allow(improper_ctypes_definitions, clippy::missing_safety_doc)]

pub mod chacha;
pub mod pcg64;
pub mod wyrand;

#[macro_export]
macro_rules! define_c_api {
	(range, $rng_name:ident, $rng_type:ty, $num_type:ty) => {
		paste::paste! {
			#[no_mangle]
			pub unsafe extern "C" fn [<$rng_name _range_ $num_type>](rng: *mut $rng_type, lower: $num_type, upper: $num_type) -> $num_type {
				let rng = &mut *rng;
				rng.generate_range(lower..=upper)
			}
		}
	};
	($name:ident, $rng_type:ty, $size:expr) => {
		paste::paste! {
			use nanorand::{BufferedRng, $rng_type, Rng};

			/// Create a new $type RNG, using system-provided entropy.
			/// This must be freed later with `free_$name(ptr)`!
			#[no_mangle]
			pub extern "C" fn [<new_ $name>]() -> *mut $rng_type {
				Box::into_raw(Box::new(<$rng_type>::new()))
			}

			/// Free a $type RNG.
			#[no_mangle]
			pub unsafe extern "C" fn [<free_ $name>](ptr: *mut $rng_type) {
				std::mem::drop(Box::from_raw(ptr));
			}

			/// Get the raw 64-bit output from the provided RNG.
			#[no_mangle]
			pub unsafe extern "C" fn [<$name _next>](rng: *mut $rng_type) -> [u8; $size] {
				(*rng).rand()
			}

			/// Fill the provided buffer with random bytes.
			#[no_mangle]
			pub unsafe extern "C" fn [<$name _fill>](
				rng: *mut $rng_type,
				buffer: *mut u8,
				buffer_length: usize,
			) {
				let rng = &mut *rng;
				let buffer = std::slice::from_raw_parts_mut(buffer, buffer_length);
				rng.fill_bytes(buffer);
			}

			define_c_api!(range, $name, $rng_type, u8);
			define_c_api!(range, $name, $rng_type, u16);
			define_c_api!(range, $name, $rng_type, u32);
			define_c_api!(range, $name, $rng_type, u64);
			define_c_api!(range, $name, $rng_type, usize);
			define_c_api!(range, $name, $rng_type, i8);
			define_c_api!(range, $name, $rng_type, i16);
			define_c_api!(range, $name, $rng_type, i32);
			define_c_api!(range, $name, $rng_type, i64);
			define_c_api!(range, $name, $rng_type, isize);

			/// Wrap a $type RNG in a buffer.
			/// This consumes the RNG, and must be freed later with `free_$name_buffered(ptr)`!
			#[no_mangle]
			pub unsafe extern "C" fn [<$name _buffered>](rng: *mut $rng_type) -> *mut BufferedRng<$rng_type, $size> {
				let rng = *Box::from_raw(rng);
				let buffered = BufferedRng::new(rng);
				Box::into_raw(Box::new(buffered))
			}

			/// Free a buffer-wrapped $type RNG.
			#[no_mangle]
			pub unsafe extern "C" fn [<free_ $name _buffered>](ptr: *mut BufferedRng<$rng_type, $size>) {
				std::mem::drop(Box::from_raw(ptr));
			}

			/// Fill the provided buffer with random bytes.
			#[no_mangle]
			pub unsafe extern "C" fn [<$name _buffered_fill>](
				rng: *mut BufferedRng<$rng_type, $size>,
				buffer: *mut u8,
				buffer_length: usize
			) {
				let rng = &mut *rng;
				let buffer = std::slice::from_raw_parts_mut(buffer, buffer_length);
				rng.fill_bytes(buffer);
			}
		}
	};
}
