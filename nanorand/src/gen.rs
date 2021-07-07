use crate::Rng;
use core::ops::{Bound, RangeBounds};

macro_rules! gen {
	($($type:ty),+) => {
		$(
			impl<R: Rng> RandomGen<R> for $type {
				fn random(r: &mut R) -> Self {
					let mut bytes = [0u8; core::mem::size_of::<$type>()];
					let mut idx = 0;
					while idx < core::mem::size_of::<$type>() {
						let random = r.rand();
						let random = random.as_ref();
						let generated = random.len().min(core::mem::size_of::<$type>());
						bytes[idx..idx + generated].copy_from_slice(&random[..generated]);
						idx += generated;
					}
					Self::from_le_bytes(bytes)
				}
			}
		)+
	};
}

macro_rules! range {
	($(($type:ty, $bigger:ty, $signed:ty)),+) => {
		$(
			impl<R: Rng> RandomRange<R> for $type {
				fn random_range<B: RangeBounds<Self>>(r: &mut R, bounds: B) -> Self {
					const BITS: $bigger = core::mem::size_of::<$type>() as $bigger * 8;
					let lower = match bounds.start_bound() {
						Bound::Included(lower) => *lower,
						Bound::Excluded(lower) => lower.saturating_add(1),
						Bound::Unbounded => <$type>::MIN,
					};
					let upper = match bounds.end_bound() {
						Bound::Included(upper) => upper.saturating_sub(lower).saturating_add(1),
						Bound::Excluded(upper) => upper.saturating_sub(lower),
						Bound::Unbounded => <$type>::MAX,
					};
					let mut value = Self::random(r);
					let mut m = (upper as $bigger).wrapping_mul(value as $bigger);
					if (m as $type) < upper {
						let t = (!upper + 1) % upper;
						while (m as $type) < t {
							value = Self::random(r);
							m = (upper as $bigger).wrapping_mul(value as $bigger);
						}
					}
					(m >> BITS) as $type + lower
				}
			}

			impl<R: Rng> RandomRange<R> for $signed {
				fn random_range<B: RangeBounds<Self>>(r: &mut R, bounds: B) -> Self {
					const SIGNED_MAPPING: $type = <$type>::MAX / 2 + 1;
					let lower = match bounds.start_bound() {
						Bound::Included(lower) => *lower,
						Bound::Excluded(lower) => lower.saturating_add(1),
						Bound::Unbounded => <$signed>::MIN
					};
					let upper = match bounds.end_bound() {
						Bound::Included(upper) => *upper,
						Bound::Excluded(upper) => upper.saturating_sub(1),
						Bound::Unbounded => <$signed>::MAX,
					};
					let lower = (lower as $type).wrapping_add(SIGNED_MAPPING);
					let upper = (upper as $type).wrapping_add(SIGNED_MAPPING);
					assert!(upper >= lower, "{} >= {}", upper, lower);
					<$type>::random_range(r, lower..=upper).wrapping_add(SIGNED_MAPPING) as $signed
				}
			}
		)+
	};
}

/// A trait used for generating a random object with an RNG,
pub trait RandomGen<R: Rng> {
	/// Return a random instance of the implementing type, from the specified RNG instance.
	fn random(r: &mut R) -> Self;
}

/// A trait used for generating a random number within a range, with an RNG,
pub trait RandomRange<R: Rng>: RandomGen<R> {
	/// Return a ranged number of the implementing type, from the specified RNG instance.
	fn random_range<B: RangeBounds<Self>>(r: &mut R, range: B) -> Self;
}

impl<R: Rng> RandomGen<R> for bool {
	fn random(r: &mut R) -> Self {
		r.rand().as_ref()[0] < 0b10000000
	}
}

impl<R: Rng> RandomGen<R> for f32 {
	fn random(r: &mut R) -> Self {
		(u32::random(r) as f32) / (u32::MAX as f32)
	}
}

impl<R: Rng> RandomGen<R> for f64 {
	fn random(r: &mut R) -> Self {
		(u64::random(r) as f64) / (u64::MAX as f64)
	}
}

gen!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
range!(
	(u8, u16, i8),
	(u16, u32, i16),
	(u32, u64, i32),
	(u64, u128, i64)
);
#[cfg(target_pointer_width = "16")]
range!((usize, u32, isize));
#[cfg(target_pointer_width = "32")]
range!((usize, u64, isize));
#[cfg(target_pointer_width = "64")]
range!((usize, u128, isize));

#[cfg(test)]
mod tests {
	use crate::{Rng, WyRand};
	#[test]
	fn ensure_unsigned_in_range() {
		let mut rng = WyRand::new();
		for _ in 0..1000 {
			let number = rng.generate_range(10_u64..=20);
			assert!(
				(10..=20).contains(&number),
				"{} was outside of 10..=20",
				number
			);

			let number = rng.generate_range(10_u64..30);
			assert!(
				(10..30).contains(&number),
				"{} was outside of 10..30",
				number
			);

			let number = rng.generate_range(512_u64..);
			assert!((512..).contains(&number), "{} was outside of 512..", number);

			let number = rng.generate_range(..1024_u64);
			assert!(
				(..1024).contains(&number),
				"{} was outside of ..1024",
				number
			);
		}
	}
	#[test]
	fn ensure_signed_in_range() {
		let mut rng = WyRand::new();
		for _ in 0..1000 {
			let number = rng.generate_range(-200..=50);
			assert!(
				(-200..=50).contains(&number),
				"{} was outside of -200..=50",
				number
			);

			let number = rng.generate_range(-200..100);
			assert!(
				(-200..100).contains(&number),
				"{} was outside of -200..100",
				number
			);

			let number = rng.generate_range(-50..);
			assert!((-50..).contains(&number), "{} was outside of -50..", number);

			let number = rng.generate_range(..512);
			assert!((..512).contains(&number), "{} was outside of ..512", number);

			let number = rng.generate_range(..-32);
			assert!((..-32).contains(&number), "{} was outside of ..-32", number);
		}
	}

	#[test]
	fn ensure_floats_generate_properly() {
		let mut rng = WyRand::new();
		for _ in 0..1000 {
			let number = rng.generate::<f32>();
			assert!(1.0 >= number, "{} was bigger than 1.0", number);
			assert!(number >= 0.0, "0 was bigger than {}", number);

			let number = rng.generate::<f64>();
			assert!(1.0 >= number, "{} was bigger than 1.0", number);
			assert!(number >= 0.0, "0 was bigger than {}", number);
		}
	}
}
