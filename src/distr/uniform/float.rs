use super::*;

/// The [Uniform distribution](https://en.wikipedia.org/wiki/Continuous_uniform_distribution) over floating point types.
///
/// # Implementation notes
///
/// Floating point samplers include the `low` argument and exclude the `high` argument regardless of which constructor was chosen.
/// Thus, when `high < low`, the numerically upper endpoint is included and the numerically lower endpoint is excluded.
/// Equal bounds produce that bound for every sample. Floating-point rounding may occasionally produce either endpoint,
/// and extreme bounds may produce non-finite results.
///
/// Accepting equal and reversed bounds is intentional. It allows callers to use
/// a signed or dynamically computed span without first ordering its endpoints,
/// and makes the sampled direction follow the direction from `low` to `high`.
/// Consequently, floating-point samplers do not return [`UniformError::EmptyRange`].
///
/// Fast floating point values are requested directly from the `Rng` then scaled and shifted into the requested range.
///
/// The fallible constructors reject non-finite inputs and non-finite values produced while calculating the sampler's scale and base.
/// This validates construction only: sampling arithmetic may still overflow or produce a non-finite value for extreme bounds.
/// The infallible constructors skip validation, allowing IEEE-754 infinities and NaNs to propagate into samples.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniformFloat<T> {
	base: T,
	scale: T,
}

impl SampleUniform for f32 {
	type Sampler = UniformFloat<f32>;
}

impl UniformSampler<f32> for UniformFloat<f32> {
	#[inline]
	fn new(low: f32, high: f32) -> UniformFloat<f32> {
		let scale = high - low;
		let base = low - scale;
		UniformFloat { base, scale }
	}

	#[inline]
	fn try_new(low: f32, high: f32) -> Result<UniformFloat<f32>, UniformError> {
		let scale = high - low;
		let base = low - scale;
		if !(base.is_finite() && scale.is_finite()) {
			return Err(UniformError::NonFinite);
		}
		Ok(UniformFloat { base, scale })
	}

	#[inline]
	fn new_inclusive(low: f32, high: f32) -> UniformFloat<f32> {
		Self::new(low, high)
	}

	#[inline]
	fn try_new_inclusive(low: f32, high: f32) -> Result<UniformFloat<f32>, UniformError> {
		Self::try_new(low, high)
	}
}

impl Distribution<f32> for UniformFloat<f32> {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> f32 {
		// Plain multiply and add to improve consistency across targets
		rand.next_f32() * self.scale + self.base
	}
}

impl SampleUniform for f64 {
	type Sampler = UniformFloat<f64>;
}

impl UniformSampler<f64> for UniformFloat<f64> {
	#[inline]
	fn new(low: f64, high: f64) -> UniformFloat<f64> {
		let scale = high - low;
		let base = low - scale;
		UniformFloat { base, scale }
	}

	#[inline]
	fn try_new(low: f64, high: f64) -> Result<UniformFloat<f64>, UniformError> {
		let scale = high - low;
		let base = low - scale;
		if !(base.is_finite() && scale.is_finite()) {
			return Err(UniformError::NonFinite);
		}
		Ok(UniformFloat { base, scale })
	}

	#[inline]
	fn new_inclusive(low: f64, high: f64) -> UniformFloat<f64> {
		Self::new(low, high)
	}

	#[inline]
	fn try_new_inclusive(low: f64, high: f64) -> Result<UniformFloat<f64>, UniformError> {
		Self::try_new(low, high)
	}
}

impl Distribution<f64> for UniformFloat<f64> {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> f64 {
		// Plain multiply and add to improve consistency across targets
		rand.next_f64() * self.scale + self.base
	}
}
