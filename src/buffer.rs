use crate::rand::Rng;
use alloc::vec::Vec;
use core::{
	default::Default,
	ops::{Deref, DerefMut},
};

/// A buffered wrapper for any [Rng] implementation.
/// It will keep unused bytes from the last call to [`Rng::rand`], and use them
/// for subsequent randomness if needed, rather than throwing them away.
///
/// ```rust
/// use nanorand::{Rng, BufferedRng, WyRand};
///
/// let mut thingy = [0u8; 5];
/// let mut rng = BufferedRng::new(WyRand::new());
/// rng.fill(&mut thingy);
/// // As WyRand generates 8 bytes of output, and our target is only 5 bytes,
/// // 3 bytes will remain in the buffer.
/// assert_eq!(rng.buffered(), 3);
/// ```
#[derive(Clone)]
pub struct BufferedRng<const OUTPUT: usize, R: Rng<OUTPUT>> {
	rng: R,
	buffer: Vec<u8>,
}

impl<const OUTPUT: usize, R: Rng<OUTPUT>> BufferedRng<OUTPUT, R> {
	/// Wraps a [`Rng`] generator in a [`BufferedRng`] instance.
	pub fn new(rng: R) -> Self {
		Self {
			rng,
			buffer: Vec::new(),
		}
	}

	/// Returns how many unused bytes are currently buffered.
	pub fn buffered(&self) -> usize {
		self.buffer.len()
	}

	/// Fills the output with random bytes, first using bytes from the internal buffer,
	/// and then using the [`Rng`] generator, and discarding unused bytes into the buffer.
	pub fn fill(&mut self, output: &mut [u8]) {
		let mut remaining = output.len();
		while remaining > 0 {
			if self.buffer.is_empty() {
				self.buffer.extend_from_slice(&self.rng.rand());
			}
			let to_copy = core::cmp::min(remaining, self.buffer.len());
			let output_len = output.len();
			let start_idx = output_len - remaining;
			output[start_idx..start_idx + to_copy].copy_from_slice(&self.buffer[..to_copy]);
			self.buffer.drain(..to_copy);
			remaining = remaining.saturating_sub(to_copy);
		}
	}
}

#[cfg(feature = "std")]
impl<const OUTPUT: usize, R: Rng<OUTPUT>> std::io::Read for BufferedRng<OUTPUT, R> {
	fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
		self.fill(output);
		Ok(output.len())
	}

	fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
		buf.extend_from_slice(&self.buffer);
		Ok(self.buffer.drain(..).count())
	}

	fn read_to_string(&mut self, _buf: &mut String) -> std::io::Result<usize> {
		panic!("attempted to read an rng into a string")
	}
}

impl<const OUTPUT: usize, R: Rng<OUTPUT>> Deref for BufferedRng<OUTPUT, R> {
	type Target = R;

	fn deref(&self) -> &Self::Target {
		&self.rng
	}
}

impl<const OUTPUT: usize, R: Rng<OUTPUT>> DerefMut for BufferedRng<OUTPUT, R> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.rng
	}
}

impl<const OUTPUT: usize, R: Rng<OUTPUT> + Default> Default for BufferedRng<OUTPUT, R> {
	fn default() -> Self {
		Self::new(R::default())
	}
}
