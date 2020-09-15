use crate::RNG;

pub trait RandomGen<R: RNG> {
	/// Return a random instance of the implementing type, from the specified RNG instance.
	fn random(r: &mut R) -> Self;
}

pub trait RandomRange<R: RNG>: RandomGen<R> {
	/// Return a ranged number of the implementing type, from the specified RNG instance.
	fn random_range(r: &mut R, lower: Self, upper: Self) -> Self;
}

impl<R: RNG> RandomGen<R> for char {
	fn random(r: &mut R) -> Self {
		loop {
			let generated = r.rand();
			let mut bytes = [0u8; core::mem::size_of::<u32>()];
			bytes
				.iter_mut()
				.zip(generated.as_ref())
				.for_each(|(a, b)| *a = *b);
			if let Some(c) = core::char::from_u32(u32::from_ne_bytes(bytes)) {
				break c;
			}
		}
	}
}

/// Boilerplate code for creating a RandomGen implementation for number types.
macro_rules! randomgen_number {
    ($($number:ty),*) => {
        $(
            impl<R: RNG> RandomGen<R> for $number {
                fn random(r: &mut R) -> Self {
                    let generated = r.rand();
                    let mut bytes = [0u8; core::mem::size_of::<$number>()];
                    bytes.iter_mut().zip(generated.as_ref()).for_each(|(a, b)| *a = *b);
                    Self::from_ne_bytes(bytes)
                }
            }

            impl<R: RNG> RandomRange<R> for $number {
                fn random_range(r: &mut R, lower: $number, upper: $number) -> Self {
                    let t = ((-(upper as i64)) % (upper as i64)) as i64;
                    let mut l: i64;
                    let mut m: i128;
                    let in_range = loop {
                        let x = Self::random(r);
                        m = (x as i128).wrapping_mul(upper as i128);
                        l = m as i64;
                        if l >= t {
                            break (m >> 64) as i64;
                        }
                    };
                    in_range.max(lower as i64) as $number
                }
            }
        )*
    }
}

randomgen_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
