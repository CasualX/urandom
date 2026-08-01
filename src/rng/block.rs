use super::*;

pub trait BlockRng {
	type Output: Copy + Default + PartialEq + Pod;

	/// Generates the next block in a byte representation that is identical on little-endian and big-endian targets.
	fn generate(&mut self, random: &mut Self::Output);
	/// Advances the generator state by a large, implementation-defined distance.
	fn jump(&mut self);
}

#[derive(Clone, Debug)]
pub struct BlockRngImpl<T: BlockRng> {
	state: T,
	index: u32,
	random: T::Output,
}

impl<T: BlockRng> BlockRngImpl<T> {
	#[inline]
	pub fn new(state: T) -> BlockRngImpl<T> {
		BlockRngImpl {
			state,
			index: !0,
			random: T::Output::default(),
		}
	}

	#[inline]
	pub fn jump(&mut self) {
		self.state.jump();
		self.index = !0;
	}
}

impl<T: BlockRng> Sealed for BlockRngImpl<T> {}

impl<T: BlockRng> Rng for BlockRngImpl<T> {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		// Generate a new block if there are no more random words
		let mut index = self.index as usize;
		if index > mem::size_of_val(&self.random) - 4 {
			self.state.generate(&mut self.random);
			index = 0;
		}
		// Fetch from the random block
		let random = pod::bytes(&self.random);
		let value = u32::from_le_bytes([random[index + 0], random[index + 1], random[index + 2], random[index + 3]]);
		self.index = (index + 4) as u32;
		value
	}

	#[inline]
	fn next_u64(&mut self) -> u64 {
		// Generate a new block if there are less than two random words
		let mut index = self.index as usize;
		if index > mem::size_of_val(&self.random) - 8 {
			self.state.generate(&mut self.random);
			index = 0;
		}
		// Fetch from the random block
		let random = pod::bytes(&self.random);
		let value = u64::from_le_bytes([
			random[index + 0], random[index + 1], random[index + 2], random[index + 3],
			random[index + 4], random[index + 5], random[index + 6], random[index + 7],
		]);
		self.index = (index + 8) as u32;
		value
	}

	#[inline(never)]
	fn fill_bytes(&mut self, mut buf: &mut [MaybeUninit<u8>]) {
		// Full blocks bypass cached bytes to avoid splicing large fills
		// This may reorder output across Rng methods, which do not share a stream-order contract
		// Keep the cache for the final partial block or a later read
		// Use a temporary block buffer due to potential alignment issues
		let mut tmp = T::Output::default();
		while buf.len() >= mem::size_of_val(&tmp) {
			self.state.generate(&mut tmp);
			let src = pod::bytes(&tmp);
			unsafe { ptr::copy_nonoverlapping(src.as_ptr(), buf.as_mut_ptr() as *mut u8, src.len()); }
			buf = &mut buf[mem::size_of_val(&tmp)..];
		}
		// Fill the remaining bytes from the cached block
		if buf.len() > 0 {
			loop {
				let random = pod::bytes(&self.random);
				let start = usize::min(self.index as usize, random.len());
				let src = &random[start..];
				let len = usize::min(src.len(), buf.len());
				unsafe { ptr::copy_nonoverlapping(src.as_ptr(), buf.as_mut_ptr() as *mut u8, len); }
				buf = &mut buf[len..];
				if buf.len() > 0 {
					self.state.generate(&mut self.random);
					self.index = 0;
				}
				else {
					self.index += len as u32;
					break;
				}
			}
		}
	}

}

#[cfg(feature = "serde")]
mod s;
