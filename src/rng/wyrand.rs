use super::*;

/// Tiny and very fast pseudorandom number generator based on [rapidhash](https://github.com/Nicoshev/rapidhash).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Wyrand {
	state: u64,
}

impl Wyrand {
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
	/// let mut rand = urandom::rng::Wyrand::new();
	/// let value: i32 = rand.next();
	/// ```
	#[inline]
	pub fn new() -> Random<Wyrand> {
		let state = util::getrandom();
		Random::wrap(Wyrand { state })
	}

	/// Creates a new instance seeded from another generator.
	///
	/// This may be useful when needing to rapidly seed many instances from a master PRNG, and to allow forking of PRNGs.
	///
	/// The master PRNG should use a sufficiently different algorithm from the child PRNG (ideally a CSPRNG) to avoid correlations between the child PRNGs.
	#[inline]
	pub fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<Wyrand> {
		Random::wrap(Wyrand { state: rand.next_u64() })
	}

	/// Creates a new instance using the given seed.
	///
	/// Implementations are required to be reproducible given the same seed.
	/// _Changing_ the implementation of this function should be considered a breaking change.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::Wyrand::from_seed(42);
	/// let value = rand.next_u32();
	/// assert_eq!(value, 3396458620);
	/// ```
	#[inline]
	pub fn from_seed(seed: u64) -> Random<Wyrand> {
		Random::wrap(Wyrand { state: seed })
	}
}

impl Rng for Wyrand {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		(wyrand(&mut self.state) >> 32) as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		wyrand(&mut self.state)
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

//----------------------------------------------------------------
// Wyrand implementation details

#[inline(always)]
fn rapid_mum(a: u64, b: u64) -> (u64, u64) {
	let r = a as u128 * b as u128;
	(r as u64, (r >> 64) as u64)
}

#[inline(always)]
fn rapid_mix(a: u64, b: u64) -> u64 {
	let (a, b) = rapid_mum(a, b);
	a ^ b
}

const P0: u64 = 0x2d358dccaa6c78a5;
const P1: u64 = 0x8bb84b93962eacc9;

#[inline]
fn wyrand(seed: &mut u64) -> u64 {
	*seed = seed.wrapping_add(P0);
	rapid_mix(*seed ^ P1, *seed)
}

#[inline]
fn jump(seed: &mut u64) {
	*seed = seed.wrapping_add(P0 << 40);
}
