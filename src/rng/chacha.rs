use super::*;

/// [`ChaCha`] with 8 rounds.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::rng::ChaCha8::new();
/// let value: i32 = rand.next();
/// ```
pub type ChaCha8 = ChaCha<8>;
impl SecureRng for ChaCha<8> {}

/// [`ChaCha`] with 12 rounds.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::rng::ChaCha12::new();
/// let value: i32 = rand.next();
/// ```
pub type ChaCha12 = ChaCha<12>;
impl SecureRng for ChaCha<12> {}

/// [`ChaCha`] with 20 rounds.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::rng::ChaCha20::new();
/// let value: i32 = rand.next();
/// ```
pub type ChaCha20 = ChaCha<20>;
impl SecureRng for ChaCha<20> {}

/// Daniel J. Bernstein's ChaCha adapted as a deterministic random number generator.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChaCha<const N: usize> {
	// The current state of the ChaCha cipher
	state: [u32; BLOCK_WORDS],
	// The Rng produces 16 words per block
	random: [u32; BLOCK_WORDS],
	// Consume the random words before producing more
	index: u32,
}

impl<const N: usize> SeedRng for ChaCha<N> where Self: SecureRng {
	#[inline]
	fn new() -> Random<ChaCha<N>> {
		let mut state = [MaybeUninit::<u32>::uninit(); BLOCK_WORDS];
		state[0].write(CONSTANT[0]);
		state[1].write(CONSTANT[1]);
		state[2].write(CONSTANT[2]);
		state[3].write(CONSTANT[3]);
		entropy::getentropy_uninit(&mut state[4..]);
		Random::from(ChaCha { state: unsafe { mem::transmute(state) }, random: [0; BLOCK_WORDS], index: !0 })
	}
	#[inline]
	fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<ChaCha<N>> {
		let mut state = [MaybeUninit::<u32>::uninit(); BLOCK_WORDS];
		state[0].write(CONSTANT[0]);
		state[1].write(CONSTANT[1]);
		state[2].write(CONSTANT[2]);
		state[3].write(CONSTANT[3]);
		rand.fill_bytes_uninit(&mut state[4..]);
		Random::from(ChaCha { state: unsafe { mem::transmute(state) }, random: [0; BLOCK_WORDS], index: !0 })
	}
	#[inline]
	fn from_seed(seed: u64) -> Random<ChaCha<N>> {
		let low = (seed & 0xffffffff) as u32;
		let high = (seed >> 32) as u32;
		let state = [
			CONSTANT[0], CONSTANT[1], CONSTANT[2], CONSTANT[3],
			low, high, low, high,
			low, high, low, high,
			1, 0, 0, 0,
		];
		Random::from(ChaCha { state, random: [0; BLOCK_WORDS], index: !0 })
	}
}

#[allow(private_bounds)]
impl<const N: usize> ChaCha<N> where Self: SecureRng {
	forward_seed_rng_impl!();
}

impl<const N: usize> Rng for ChaCha<N> where Self: SecureRng {
	#[inline(never)]
	fn next_u32(&mut self) -> u32 {
		let mut index = self.index as usize;
		// Generate a new block if there are no more random words
		if index >= BLOCK_WORDS {
			chacha_block::<N>(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fetch a word from the random block
		let value = self.random[index];
		self.index = (index + 1) as u32;
		value
	}
	#[inline(never)]
	fn next_u64(&mut self) -> u64 {
		let mut index = self.index as usize;
		// Generate a new block if there are less than two random words
		if index >= BLOCK_WORDS - 1 {
			chacha_block::<N>(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fetch two words from the random block
		let low = self.random[index + 0] as u64;
		let high = self.random[index + 1] as u64;
		self.index = (index + 2) as u32;
		high << 32 | low
	}
	#[inline(never)]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		if buf.as_ptr() as usize % 4 == 0 && buf.len() % 4 == 0 {
			unsafe { self.fill_aligned(buf.as_mut_ptr() as *mut u32, buf.len() / 4) };
		}
		else {
			unsafe { self.fill_unaligned(buf.as_mut_ptr() as *mut u8, buf.len()) };
		}
	}
	#[inline]
	fn jump(&mut self) {
		increment_stream(&mut self.state);
		self.index = !0;
	}
}

impl<const N: usize> ChaCha<N> {
	#[inline(always)]
	unsafe fn fill_aligned(&mut self, mut dest: *mut u32, mut len: usize) {
		// Fill directly from the generator
		while len >= BLOCK_WORDS {
			let block = dest as *mut [u32; BLOCK_WORDS];
			chacha_block::<N>(&mut self.state, &mut *block);
			dest = dest.add(BLOCK_WORDS);
			len -= BLOCK_WORDS;
		}
		// Generate a new block if there are not enough words remaining
		let max_index = BLOCK_WORDS - len;
		let mut index = self.index as usize;
		if index > max_index {
			chacha_block::<N>(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fill the remaining words from the random block
		while len > 0 {
			dest.write(self.random[index]);
			dest = dest.add(1);
			len -= 1;
			index += 1;
		}
		self.index = index as u32;
	}
	#[inline(always)]
	unsafe fn fill_unaligned(&mut self, mut dest: *mut u8, mut len: usize) {
		// Fill directly from the generator
		// Use a temporary block buffer due to potential alignment issues
		let mut tmp = [0; BLOCK_WORDS];
		while len >= BLOCK_SIZE {
			chacha_block::<N>(&mut self.state, &mut tmp);
			ptr::copy_nonoverlapping(tmp.as_ptr() as *const u8, dest, BLOCK_SIZE);
			dest = dest.add(BLOCK_SIZE);
			len -= BLOCK_SIZE;
		}
		// Generate a new block if there are not enough words remaining
		let max_index = (BLOCK_SIZE + 3 - len) / 4;
		let mut index = self.index as usize;
		if index > max_index {
			chacha_block::<N>(&mut self.state, &mut self.random);
			index = 0;
		}
		// Fill the remaining words from the random block
		let src = dataview::bytes(&self.random[index..]);
		for i in 0..len {
			dest.add(i).write(src[i]);
		}
		index += (len + 3) / 4;
		self.index = index as u32;
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
	let counter = (state[15] as u128) << 96 | (state[14] as u128) << 64 | (state[13] as u128) << 32 | (state[12] as u128) << 0;
	let counter = counter.wrapping_add(1);
	state[12] = (counter >> 0) as u32;
	state[13] = (counter >> 32) as u32;
	state[14] = (counter >> 64) as u32;
	state[15] = (counter >> 96) as u32;
}

#[inline]
fn increment_stream(state: &mut [u32; 16]) {
	let stream = (state[15] as u64) << 32 | (state[14] as u64) << 0;
	let stream = stream.wrapping_add(1);
	state[14] = (stream >> 0) as u32;
	state[15] = (stream >> 32) as u32;
}

cfg_if::cfg_if! {
	if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))] {
		mod sse2;
		use self::sse2::block as chacha_block;
	}
	else {
		mod slp;
		use self::slp::block as chacha_block;
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
	chacha_block::<20>(&mut state, &mut result);
	assert_eq!(expected, result);
}
