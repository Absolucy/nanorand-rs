use crate::RNG;

/// A trait used for generating a random object with an RNG,
pub trait RandomGen<R: RNG> {
	/// Return a random instance of the implementing type, from the specified RNG instance.
	fn random(r: &mut R) -> Self;
}

/// A trait used for generating a random number within a range, with an RNG,
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
/// Uses Lemire's debiased integer multiplication method.
macro_rules! randomgen_number {
    ($(($unsigned:ty, $signed:ty, $bigger_unsigned:ty, $bigger_signed:ty)),*) => {
        $(
            impl<R: RNG> RandomGen<R> for $unsigned {
                fn random(r: &mut R) -> Self {
                    let generated = r.rand();
                    let mut bytes = [0u8; core::mem::size_of::<$unsigned>()];
                    bytes.iter_mut().zip(generated.as_ref()).for_each(|(a, b)| *a = *b);
                    Self::from_ne_bytes(bytes)
                }
            }

            impl<R: RNG> RandomRange<R> for $unsigned {
                fn random_range(r: &mut R, lower: $unsigned, upper: $unsigned) -> Self {
                    let t = ((-(upper as $signed)) % (upper as $signed)) as $unsigned;
                    let in_range = loop {
                        let x = Self::random(r);
                        let m = (x as $bigger_unsigned).wrapping_mul(upper as $bigger_unsigned);
                        if (m as $unsigned) >= t {
                            break (m >> (::core::mem::size_of::<$unsigned>() * 8)) as $unsigned;
                        }
                    };
                    in_range.max(lower)
                }
            }

            impl<R: RNG> RandomGen<R> for $signed {
                fn random(r: &mut R) -> Self {
                    let generated = r.rand();
                    let mut bytes = [0u8; core::mem::size_of::<$signed>()];
                    bytes.iter_mut().zip(generated.as_ref()).for_each(|(a, b)| *a = *b);
                    Self::from_ne_bytes(bytes)
                }
            }

            impl<R: RNG> RandomRange<R> for $signed {
                fn random_range(r: &mut R, lower: $signed, upper: $signed) -> Self {
                    let t = ((-(upper as $signed)) % (upper as $signed));
                    let in_range = loop {
                        let x = Self::random(r);
                        let m = (x as $bigger_signed).wrapping_mul(upper as $bigger_signed);
                        if (m as $signed) >= t {
                            break (m >> (::core::mem::size_of::<$signed>() * 8)) as $signed;
                        }
                    };
                    in_range.max(lower)
                }
            }
        )*
    }
}

randomgen_number!(
	(u8, i8, u16, i16),
	(u16, i16, u32, i32),
	(u32, i32, u64, i16),
	(u64, i64, u128, i128),
	(u128, i128, u128, i128),
	(usize, isize, u128, i128)
);
