use dataview::Pod;
use crate::{Random, Rng};
use super::SeedRng;

/**
This is xoshiro256 1.0, one of our all-purpose, rock-solid generators.

It has excellent (sub-ns) speed, a state (256 bits) that is large enough for any parallel application, and it passes all tests we are aware of.

The state must be seeded so that it is not everywhere zero.
If you have a 64-bit seed, we suggest to seed a SplitMix64 generator and use its output to fill s.
*/
///
/// # Examples
///
/// ```
/// let mut rng = urandom::rng::Xoshiro256::new();
/// let value: i32 = rng.next();
/// ```
#[derive(Clone, Debug)]
pub struct Xoshiro256 {
	state: [u64; 4],
}

impl SeedRng for Xoshiro256 {
	#[inline]
	fn new() -> Random<Xoshiro256> {
		let mut state = [0u64; 4];
		super::getentropy(state.as_bytes_mut());
		Random(Xoshiro256 { state })
	}
	#[inline]
	fn from_rng<R: Rng + ?Sized>(rng: &mut Random<R>) -> Random<Xoshiro256> {
		let mut state = [0u64; 4];
		rng.fill_u64(&mut state);
		Random(Xoshiro256 { state })
	}
	fn from_seed(seed: u64) -> Random<Xoshiro256> {
		SeedRng::from_rng(&mut Random(super::SplitMix64(seed)))
	}
}

forward_seed_rng_impl!(Xoshiro256);

impl Rng for Xoshiro256 {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		(next_plus(&mut self.state) >> 32) as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		next_plusplus(&mut self.state)
	}
	#[inline]
	fn next_f32(&mut self) -> f32 {
		crate::impls::rng_f32(self.next_u32())
	}
	#[inline]
	fn next_f64(&mut self) -> f64 {
		crate::impls::rng_f64(next_plus(&mut self.state))
	}
	#[inline(never)]
	fn fill_u32(&mut self, buffer: &mut [u32]) {
		*self = crate::impls::fill_u32(self.clone(), buffer);
	}
	#[inline(never)]
	fn fill_u64(&mut self, buffer: &mut [u64]) {
		*self = crate::impls::fill_u64(self.clone(), buffer);
	}
	#[inline(never)]
	fn fill_bytes(&mut self, buffer: &mut [u8]) {
		*self = crate::impls::fill_bytes(self.clone(), buffer);
	}
	#[inline]
	fn jump(&mut self) {
		jump(&mut self.state)
	}
}

//----------------------------------------------------------------
// Xoshiro256 implementation details

#[inline]
fn next_plusplus(s: &mut [u64; 4]) -> u64 {
	let result = u64::wrapping_add(u64::wrapping_add(s[0], s[3]).rotate_left(23), s[0]);

	let t = s[1] << 17;

	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];

	s[2] ^= t;

	s[3] = s[3].rotate_left(45);

	return result;
}
#[inline]
fn next_plus(s: &mut [u64; 4]) -> u64 {
	let result = u64::wrapping_add(s[0], s[3]);

	let t = s[1] << 17;

	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];

	s[2] ^= t;

	s[3] = s[3].rotate_left(45);

	return result;
}
#[allow(dead_code)]
fn next_starstar(s: &mut [u64; 4]) -> u64 {
	let result = u64::wrapping_mul(u64::wrapping_mul(s[1], 5).rotate_left(7), 9);

	let t = s[1] << 17;

	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];

	s[2] ^= t;

	s[3] = s[3].rotate_left(45);

	return result;
}
#[inline(never)]
fn jump(s: &mut [u64; 4]) {
	static JUMP: [u64; 4] = [0x180ec6d33cfd0aba, 0xd5a61266f0c9392c, 0xa9582618e03fc9aa, 0x39abdc4529b1661c];

	let mut s0 = 0;
	let mut s1 = 0;
	let mut s2 = 0;
	let mut s3 = 0;
	for i in 0..4 {
		for b in 0..64 {
			if (JUMP[i] & (1 << b)) != 0 {
				s0 ^= s[0];
				s1 ^= s[1];
				s2 ^= s[2];
				s3 ^= s[3];
			}
			next_plusplus(s);
		}
	}
	s[0] = s0;
	s[1] = s1;
	s[2] = s2;
	s[3] = s3;
}
