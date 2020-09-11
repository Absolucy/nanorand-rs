use crate::RNG;

pub trait RandomGen<R: RNG> {
	/// Return a random instance of the implementing type, from the specified RNG instance.
	fn generate(r: &mut R) -> Self;
	/*
	/// Return a random instance of the implementing type, from the global state.
	fn generate_global() -> Self;
	*/
}

/// Boilerplate code for creating a RandomGen implementation for number types.
macro_rules! randomgen_number {
    ($($number:tt),*) => {
        $(
            impl<R: RNG> RandomGen<R> for $number {
                fn generate(r: &mut R) -> Self {
                    let generated = r.rand();
                    let mut bytes = [0u8; std::mem::size_of::<$number>()];
                    bytes.iter_mut().zip(generated.as_ref()).for_each(|(a, b)| *a = *b);
                    Self::from_ne_bytes(bytes)
                }
            }
        )*
    }
}

randomgen_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
