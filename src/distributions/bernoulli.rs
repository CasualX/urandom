use crate::{Distribution, Random, Rng};

/// Sample from the [Bernoulli distribution](https://en.wikipedia.org/wiki/Bernoulli_distribution).
#[derive(Copy, Clone, Debug)]
pub struct Bernoulli {
	p: f64,
}

impl Bernoulli {
	/// Construct a new `Bernoulli` with the given probability of success `p`.
	///
	/// # Precision
	///
	/// For p >= 1.0, the resulting distribution will always generate true.
	/// For p <= 0.0, the resulting distribution will always generate false.
	pub const fn new(p: f64) -> Bernoulli {
		Bernoulli { p }
	}
}

impl Distribution<bool> for Bernoulli {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> bool {
		<crate::distributions::Float01 as Distribution<f64>>::sample(&crate::distributions::Float01, rng) <= self.p
	}
}

#[test]
fn test_trivial() {
	let mut rng = crate::new();
	let always_false = Bernoulli::new(0.0);
	let always_true = Bernoulli::new(1.0);
	for _ in 0..5 {
		assert_eq!(rng.sample::<bool, _>(&always_false), false);
		assert_eq!(rng.sample::<bool, _>(&always_true), true);
		assert_eq!(Distribution::<bool>::sample(&always_false, &mut rng), false);
		assert_eq!(Distribution::<bool>::sample(&always_true, &mut rng), true);
	}
}
