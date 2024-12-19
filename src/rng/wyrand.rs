use super::*;

/// Tiny and very fast pseudorandom number generator based on [rapidhash](https://github.com/Nicoshev/rapidhash).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Wyrand {
	state: u64,
}

impl SeedRng for Wyrand {
	#[inline]
	fn new() -> Random<Wyrand> {
		let state = util::getrandom();
		Random::from(Wyrand { state })
	}
	#[inline]
	fn from_rng<R: ?Sized + Rng>(rand: &mut Random<R>) -> Random<Wyrand> {
		Random::from(Wyrand { state: rand.next_u64() })
	}
	#[inline]
	fn from_seed(seed: u64) -> Random<Wyrand> {
		Random::from(Wyrand { state: seed })
	}
}

impl Wyrand {
	forward_seed_rng_impl!();
}

impl Rng for Wyrand {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		wyrand(&mut self.state) as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		wyrand(&mut self.state)
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
// Wyrand implementation details

#[inline(always)]
const fn rapid_mum(a: u64, b: u64) -> (u64, u64) {
	let r = a as u128 * b as u128;
	(r as u64, (r >> 64) as u64)
}

#[inline(always)]
const fn rapid_mix(a: u64, b: u64) -> u64 {
	let (a, b) = rapid_mum(a, b);
	a ^ b
}

const P0: u64 = 0x2d358dccaa6c78a5;
const P1: u64 = 0x8bb84b93962eacc9;

#[inline]
const fn wyrand(seed: &mut u64) -> u64 {
	*seed = seed.wrapping_add(P0);
	rapid_mix(*seed ^ P1, *seed)
}

#[inline]
const fn jump(seed: &mut u64) {
	*seed = seed.wrapping_add(P0 << 40);
}
