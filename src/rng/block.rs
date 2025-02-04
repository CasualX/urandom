use super::*;

pub trait BlockRng {
	type Output: Copy + Default + PartialEq + dataview::Pod;

	fn generate(&mut self, random: &mut Self::Output);
	fn jump(&mut self);
}

#[inline]
fn bytes<T: dataview::Pod + ?Sized>(value: &T) -> &[u8] {
	unsafe { core::slice::from_raw_parts(value as *const T as *const u8, mem::size_of_val(value)) }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockRngImpl<T: BlockRng> {
	state: T,
	#[cfg_attr(feature = "serde", serde(default = "default_index::<T>", skip_serializing_if = "is_index_oob::<T>"))]
	index: u32,
	#[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
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
}

impl<T: BlockRng> Rng for BlockRngImpl<T> {
	#[inline(never)]
	fn next_u32(&mut self) -> u32 {
		// Generate a new block if there are no more random words
		let mut index = self.index as usize;
		if index > mem::size_of_val(&self.random) - 4 {
			self.state.generate(&mut self.random);
			index = 0;
		}
		// Fetch from the random block
		let random = bytes(&self.random);
		let value = u32::from_le_bytes([random[index + 0], random[index + 1], random[index + 2], random[index + 3]]);
		self.index = (index + 4) as u32;
		value
	}

	#[inline(never)]
	fn next_u64(&mut self) -> u64 {
		// Generate a new block if there are less than two random words
		let mut index = self.index as usize;
		if index > mem::size_of_val(&self.random) - 8 {
			self.state.generate(&mut self.random);
			index = 0;
		}
		// Fetch from the random block
		let random = bytes(&self.random);
		let value = u64::from_le_bytes([
			random[index + 0], random[index + 1], random[index + 2], random[index + 3],
			random[index + 4], random[index + 5], random[index + 6], random[index + 7],
		]);
		self.index = (index + 8) as u32;
		value
	}

	#[inline(never)]
	fn fill_bytes(&mut self, mut buf: &mut [MaybeUninit<u8>]) {
		// Fill directly from the generator
		// Use a temporary block buffer due to potential alignment issues
		let mut tmp = T::Output::default();
		while buf.len() >= mem::size_of_val(&tmp) {
			self.state.generate(&mut tmp);
			unsafe { ptr::copy_nonoverlapping(&tmp as *const _ as *const u8, buf.as_mut_ptr() as *mut u8, mem::size_of_val(&tmp)); }
			buf = &mut buf[mem::size_of_val(&tmp)..];
		}
		// Fill the remaining bytes from the random block
		if buf.len() > 0 {
			loop {
				let random = bytes(&self.random);
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

	#[inline]
	fn jump(&mut self) {
		self.state.jump();
		self.index = !0;
	}
}

cfg_if::cfg_if! {
	if #[cfg(feature = "serde")] {
		fn is_default<T: Default + PartialEq>(value: &T) -> bool {
			*value == T::default()
		}
		fn is_index_oob<T: BlockRng>(value: &u32) -> bool {
			*value >= mem::size_of::<T::Output>() as u32
		}
		fn default_index<T: BlockRng>() -> u32 {
			!0
		}
	}
}
