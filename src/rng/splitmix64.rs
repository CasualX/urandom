use super::*;

/// Java 8's SplittableRandom generator.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::rng::SplitMix64::new();
/// let value: i32 = rand.next();
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct SplitMix64 {
	state: u64,
}

impl SeedRng for SplitMix64 {
	#[inline]
	fn new() -> Random<SplitMix64> {
		let state = util::getrandom();
		Random::from(SplitMix64 { state })
	}
	#[inline]
	fn from_rng<R: ?Sized + Rng>(rand: &mut Random<R>) -> Random<SplitMix64> {
		Random::from(SplitMix64 { state: rand.next_u64() })
	}
	#[inline]
	fn from_seed(seed: u64) -> Random<SplitMix64> {
		Random::from(SplitMix64 { state: seed })
	}
}

impl SplitMix64 {
	forward_seed_rng_impl!();
}

impl Rng for SplitMix64 {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		next(&mut self.state) as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		next(&mut self.state)
	}
	#[inline(never)]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		*self = util::fill_bytes(self.clone(), buf);
	}
	#[inline]
	fn jump(&mut self) {
		jump(&mut self.state)
	}
}

//----------------------------------------------------------------
// SplitMix64 implementation details

const GOLDEN_GAMMA: u64 = 0x9e3779b97f4a7c15;

#[inline]
const fn next(x: &mut u64) -> u64 {
	*x = x.wrapping_add(GOLDEN_GAMMA);
	mix64(*x)
}
#[inline]
const fn jump(x: &mut u64) {
	*x = x.wrapping_add(GOLDEN_GAMMA << 40);
}

// https://zimbry.blogspot.com/2011/09/better-bit-mixing-improving-on.html
#[inline]
const fn mix64(mut z: u64) -> u64 {
	z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
	z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
	return z ^ (z >> 31);
}
