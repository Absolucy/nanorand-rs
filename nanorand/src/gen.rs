use crate::Rng;

/// A trait used for generating a random object with an RNG,
pub trait RandomGen<R: Rng> {
	/// Return a random instance of the implementing type, from the specified RNG instance.
	fn random(r: &mut R) -> Self;
}

/// A trait used for generating a random number within a range, with an RNG,
pub trait RandomRange<R: Rng>: RandomGen<R> {
	/// Return a ranged number of the implementing type, from the specified RNG instance.
	fn random_range(r: &mut R, lower: Self, upper: Self) -> Self;
}

impl<R: Rng> RandomGen<R> for char {
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

impl<R: Rng> RandomGen<R> for bool {
	fn random(r: &mut R) -> Self {
		r.rand().as_ref()[0] < 0b10000000
	}
}

impl<R: Rng> RandomGen<R> for u64 {
	fn random(r: &mut R) -> Self {
		let generated = r.rand();
		let mut bytes = [0u8; core::mem::size_of::<u64>()];
		bytes
			.iter_mut()
			.zip(generated.as_ref())
			.for_each(|(a, b)| *a = *b);
		Self::from_le_bytes(bytes)
	}
}

impl<R: Rng> RandomRange<R> for u64 {
	fn random_range(r: &mut R, lower: u64, upper: u64) -> Self {
		assert!(upper > lower);
		let range = upper.saturating_add(1);
		let mut x = Self::random(r);
		let mut m = (x as u128).wrapping_mul(range as u128);
		if (m as u64) < range {
			let mut t = u64::MAX.wrapping_sub(range);
			if t >= range {
				t -= range;
				if t >= range {
					t %= range;
				}
			}
			while (m as u64) < t {
				x = Self::random(r);
				m = (x as u128).wrapping_mul(range as u128);
			}
		}
		((m >> 64) as u64).max(lower)
	}
}

#[cfg(target_pointer_width = "64")]
impl<R: Rng> RandomGen<R> for usize {
	fn random(r: &mut R) -> Self {
		r.generate::<u64>() as usize
	}
}
#[cfg(target_pointer_width = "64")]
impl<R: Rng> RandomRange<R> for usize {
	fn random_range(r: &mut R, lower: usize, upper: usize) -> Self {
		r.generate_range::<u64>(lower as u64, upper as u64) as usize
	}
}

#[cfg(target_pointer_width = "32")]
impl<R: Rng> RandomGen<R> for usize {
	fn random(r: &mut R) -> Self {
		r.generate::<u32>() as usize
	}
}
#[cfg(target_pointer_width = "32")]
impl<R: Rng> RandomRange<R> for usize {
	fn random_range(r: &mut R, lower: usize, upper: usize) -> Self {
		r.generate_range::<u32>(lower as u32, upper as u32) as usize
	}
}

#[cfg(target_pointer_width = "16")]
impl<R: Rng> RandomGen<R> for usize {
	fn random(r: &mut R) -> Self {
		r.generate::<u16>() as usize
	}
}
#[cfg(target_pointer_width = "16")]
impl<R: Rng> RandomRange<R> for usize {
	fn random_range(r: &mut R, lower: usize, upper: usize) -> Self {
		r.generate_range::<u16>(lower as u16, upper as u16) as usize
	}
}

impl<R: Rng> RandomGen<R> for u32 {
	fn random(r: &mut R) -> Self {
		r.generate::<u64>() as u32
	}
}

impl<R: Rng> RandomRange<R> for u32 {
	fn random_range(r: &mut R, lower: u32, upper: u32) -> Self {
		r.generate_range::<u64>(lower as u64, upper as u64) as u32
	}
}

impl<R: Rng> RandomGen<R> for u16 {
	fn random(r: &mut R) -> Self {
		r.generate::<u64>() as u16
	}
}

impl<R: Rng> RandomRange<R> for u16 {
	fn random_range(r: &mut R, lower: u16, upper: u16) -> Self {
		r.generate_range::<u64>(lower as u64, upper as u64) as u16
	}
}

impl<R: Rng> RandomGen<R> for u8 {
	fn random(r: &mut R) -> Self {
		r.generate::<u64>() as u8
	}
}

impl<R: Rng> RandomRange<R> for u8 {
	fn random_range(r: &mut R, lower: u8, upper: u8) -> Self {
		r.generate_range::<u64>(lower as u64, upper as u64) as u8
	}
}

impl<R: Rng> RandomRange<R> for char {
	fn random_range(r: &mut R, lower: char, upper: char) -> Self {
		loop {
			let ret = r.generate_range::<u64>(lower as u64, upper as u64) as u32;
			if let Some(c) = core::char::from_u32(ret) {
				break c;
			}
		}
	}
}

impl<R: Rng> RandomGen<R> for f32 {
	fn random(r: &mut R) -> Self {
		r.generate::<u32>() as f32
	}
}

impl<R: Rng> RandomGen<R> for f64 {
	fn random(r: &mut R) -> Self {
		r.generate::<u64>() as f64
	}
}
