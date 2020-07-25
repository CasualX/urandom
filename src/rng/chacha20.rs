use dataview::Pod;
use crate::{Random, Rng};
use super::SeedRng;

/// Daniel J. Bernstein's ChaCha20 adapted as a deterministic random number generator.
///
/// # Examples
///
/// ```
/// let mut rng = urandom::rng::ChaCha20::new();
/// let value: i32 = rng.next();
/// ```
#[derive(Clone, Debug)]
pub struct ChaCha20 {
	// The current state of the ChaCha20 cipher
	state: [u32; BLOCK_WORDS],
	// The Rng produces 16 words per block
	random: [u32; BLOCK_WORDS],
	// Consume the random words before producing more
	index: u32,
}

impl SeedRng for ChaCha20 {
	#[inline]
	fn new() -> Random<ChaCha20> {
		let mut state = [
			CONSTANT[0], CONSTANT[1], CONSTANT[2], CONSTANT[3],
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		];
		super::getentropy(state[4..].as_bytes_mut());
		Random(ChaCha20 { state, random: [0; BLOCK_WORDS], index: !0 })
	}
	#[inline]
	fn from_rng<R: Rng + ?Sized>(rng: &mut Random<R>) -> Random<ChaCha20> {
		let mut state = [
			CONSTANT[0], CONSTANT[1], CONSTANT[2], CONSTANT[3],
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		];
		rng.fill_u32(&mut state[4..]);
		Random(ChaCha20 { state, random: [0; BLOCK_WORDS], index: !0 })
	}
	#[inline]
	fn from_seed(seed: u64) -> Random<ChaCha20> {
		let low = (seed & 0xffffffff) as u32;
		let high = (seed >> 32) as u32;
		Random(ChaCha20 {
			state: [
				CONSTANT[0], CONSTANT[1], CONSTANT[2], CONSTANT[3],
				low, high, low, high,
				low, high, low, high,
				1, 0, 0, 0,
			],
			random: [0; BLOCK_WORDS],
			index: !0,
		})
	}
}

forward_seed_rng_impl!(ChaCha20);

impl Rng for ChaCha20 {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		let mut index = self.index as usize;
		// Generate a new block if there are no more random words
		if index >= BLOCK_WORDS {
			chacha20_block(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fetch a word from the random block
		let value = self.random[index];
		index += 1;
		self.index = index as u32;
		value
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		let mut index = self.index as usize;
		// Generate a new block if there are less than two random words
		if index >= BLOCK_WORDS - 1 {
			chacha20_block(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fetch two words from the random block
		let low = self.random[index + 0] as u64;
		let high = self.random[index + 1] as u64;
		index += 2;
		self.index = index as u32;
		high << 32 | low
	}
	fn fill_u32(&mut self, mut buffer: &mut [u32]) {
		// Fill directly from the generator
		while buffer.len() >= BLOCK_WORDS {
			let block = buffer.as_data_view_mut().read_mut(0);
			chacha20_block(&mut self.state, block);
			buffer = &mut buffer[BLOCK_WORDS..];
		}
		// Generate a new block if there are not enough words remaining
		let max_index = BLOCK_WORDS - buffer.len();
		let mut index = self.index as usize;
		if index > max_index {
			chacha20_block(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fill the remaining words from the random block
		while buffer.len() > 0 {
			buffer[0] = self.random[index];
			index += 1;
		}
		self.index = index as u32;
	}
	#[inline]
	fn fill_u64(&mut self, buffer: &mut [u64]) {
		// Implement via fill_u32
		self.fill_u32(buffer.as_data_view_mut().slice_tail_mut(0));
	}
	fn fill_bytes(&mut self, mut buffer: &mut [u8]) {
		// Fill directly from the generator
		// Use a temporary block buffer due to potential alignment issues
		let mut tmp = [0; BLOCK_WORDS];
		while buffer.len() >= BLOCK_SIZE {
			chacha20_block(&mut self.state, &mut tmp);
			buffer[..BLOCK_SIZE].copy_from_slice(tmp.as_bytes());
			buffer = &mut buffer[BLOCK_SIZE..];
		}
		// Generate a new block if there are not enough words remaining
		let max_index = (BLOCK_SIZE + 3 - buffer.len()) / 4;
		let mut index = self.index as usize;
		if index > max_index {
			chacha20_block(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fill the remaining words from the random block
		let src = self.random[index..].as_bytes();
		for i in 0..buffer.len() {
			buffer[i] = src[i];
		}
		index += (buffer.len() + 3) / 4;
		self.index = index as u32;
	}
	fn jump(&mut self) {
		let mut tmp = [0; 16];
		chacha20_block(&mut self.state, &mut tmp);
		for i in 0..8 {
			self.state[i + 4] ^= tmp[i];
			self.state[i + 4] ^= tmp[i + 8];
		}
		self.index = !0;
	}
}

//----------------------------------------------------------------
// ChaCha20 implementation details
// https://cr.yp.to/chacha/chacha-20080128.pdf
// http://loup-vaillant.fr/tutorials/chacha20-design

const CONSTANT: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];
const BLOCK_WORDS: usize = 16;
const BLOCK_SIZE: usize = 16 * 4;

#[inline]
fn increment_counter(state: &mut [u32; 16]) {
	// Yeah this can technically be written in pure safe code but that would be a chore...
	use core::ptr::{read_unaligned, write_unaligned};
	unsafe {
		let counter = (state as *mut _ as *mut u128).offset(3);
		write_unaligned(counter, u128::from_le(read_unaligned(counter).to_le().wrapping_add(1)));
	}
}

cfg_if::cfg_if! {
	if #[cfg(all(target_arch = "x86", target_feature = "sse2"))] {
		mod x86;
		use self::x86::block as chacha20_block;
	}
	else if #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))] {
		mod x86;
		use self::x86::block as chacha20_block;
	}
	else {
		mod slp;
		use self::slp::block as chacha20_block;
	}
}

#[test]
fn chacha20_selftest() {
	let mut state = [
		CONSTANT[0], CONSTANT[1], CONSTANT[2], CONSTANT[3],
		0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c,
		0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
		0x00000001, 0x09000000, 0x4a000000, 0x00000000,
	];
	let expected = [
		0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3,
		0xc7f4d1c7, 0x0368c033, 0x9aaa2204, 0x4e6cd4c3,
		0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9,
		0xd19c12b5, 0xb94e16de, 0xe883d0cb, 0x4e3c50a2,
	];
	let mut result = [0; 16];
	chacha20_block(&mut state, &mut result);
	assert_eq!(expected, result);
}
