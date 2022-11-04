use crate::{Random, Rng};
use super::SeedRng;

/// Java 8's SplittableRandom generator.
///
/// # Examples
///
/// ```
/// let mut rng = urandom::rng::SplitMix64::new();
/// let value: i32 = rng.next();
/// ```
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct SplitMix64(pub(crate) u64);

impl SeedRng for SplitMix64 {
	#[inline]
	fn new() -> Random<SplitMix64> {
		let mut state = 0u64;
		super::getentropy(dataview::bytes_mut(&mut state));
		Random(SplitMix64(state))
	}
	#[inline]
	fn from_rng<R: ?Sized + Rng>(rng: &mut Random<R>) -> Random<SplitMix64> {
		Random(SplitMix64(rng.next_u64()))
	}
	#[inline]
	fn from_seed(seed: u64) -> Random<SplitMix64> {
		Random(SplitMix64(seed))
	}
}

forward_seed_rng_impl!(SplitMix64);

impl Rng for SplitMix64 {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		next(&mut self.0) as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		next(&mut self.0)
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
		jump(&mut self.0)
	}
}

//----------------------------------------------------------------
// SplitMix64 implementation details

const GOLDEN_GAMMA: u64 = 0x9e3779b97f4a7c15;

#[inline]
fn next(x: &mut u64) -> u64 {
	*x = x.wrapping_add(GOLDEN_GAMMA);
	mix64(*x)
}
#[inline]
fn jump(x: &mut u64) {
	*x = x.wrapping_add(GOLDEN_GAMMA << 40);
}

// https://zimbry.blogspot.com/2011/09/better-bit-mixing-improving-on.html
#[inline]
const fn mix64(mut z: u64) -> u64 {
	z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
	z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
	return z ^ (z >> 31);
}
