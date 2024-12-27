use super::*;

/// The [Bernoulli distribution](https://en.wikipedia.org/wiki/Bernoulli_distribution).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bernoulli {
	p: f64,
}

impl Bernoulli {
	/// Constructs a new `Bernoulli` with the given probability of success `p`.
	///
	/// # Precision
	///
	/// For p >= 1.0, the resulting distribution will always generate true.
	/// For p <= 0.0, the resulting distribution will always generate false.
	#[inline]
	pub const fn new(p: f64) -> Bernoulli {
		Bernoulli { p }
	}

	/// Returns the probability (`p`) of the distribution.
	#[inline]
	pub const fn p(&self) -> f64 {
		self.p
	}
}

impl Distribution<bool> for Bernoulli {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> bool {
		<Float01 as Distribution<f64>>::sample(&Float01, rand) <= self.p
	}
}

#[test]
fn test_trivial() {
	let mut rand = crate::new();
	let always_false = Bernoulli::new(0.0);
	let always_true = Bernoulli::new(1.0);
	for _ in 0..5 {
		assert_eq!(rand.sample::<bool, _>(&always_false), false);
		assert_eq!(rand.sample::<bool, _>(&always_true), true);
		assert_eq!(Distribution::<bool>::sample(&always_false, &mut rand), false);
		assert_eq!(Distribution::<bool>::sample(&always_true, &mut rand), true);
	}
}
