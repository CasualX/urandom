use super::*;

/// Fast RNG, with 64 bits of state, that can be used to initialize the state of other generators.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct SplitMix64 {
	state: u64,
}

impl SplitMix64 {
	/// Creates a new instance seeded securely from system entropy.
	///
	/// This method is the recommended way to construct PRNGs since it is convenient and secure.
	///
	/// # Panics
	///
	/// If [`getentropy`] is unable to provide secure entropy this method will panic.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::SplitMix64::new();
	/// let value: i32 = rand.next();
	/// ```
	#[inline]
	pub fn new() -> Random<SplitMix64> {
		let state = util::getrandom();
		Random::wrap(SplitMix64 { state })
	}

	/// Creates a new instance seeded from another generator.
	///
	/// This may be useful when needing to rapidly seed many instances from a master PRNG, and to allow forking of PRNGs.
	///
	/// The master PRNG should use a sufficiently different algorithm from the child PRNG (ideally a CSPRNG) to avoid correlations between the child PRNGs.
	#[inline]
	pub fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<SplitMix64> {
		Random::wrap(SplitMix64 { state: rand.next_u64() })
	}

	/// Creates a new instance using the given seed.
	///
	/// Implementations are required to be reproducible given the same seed.
	/// _Changing_ the implementation of this function should be considered a breaking change.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::SplitMix64::from_seed(42);
	/// let value = rand.next_u32();
	/// assert_eq!(value, 3184996902);
	/// ```
	#[inline]
	pub fn from_seed(seed: u64) -> Random<SplitMix64> {
		Random::wrap(SplitMix64 { state: seed })
	}
}

impl Rng for SplitMix64 {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		(next(&mut self.state) >> 32) as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		next(&mut self.state)
	}
	#[inline(never)]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		let mut rng = self.clone();
		util::rng_fill_bytes(&mut rng, buf);
		*self = rng;
	}
	#[inline]
	fn jump(&mut self) {
		jump(&mut self.state)
	}
}

cfg_if::cfg_if! {
	if #[cfg(feature = "serde")] {
		#[test]
		fn serde() {
			util::check_serde_initial_state(SplitMix64::new());
			util::check_serde_middle_state(SplitMix64::new());
		}
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
fn mix64(mut z: u64) -> u64 {
	z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
	z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
	return z ^ (z >> 31);
}
