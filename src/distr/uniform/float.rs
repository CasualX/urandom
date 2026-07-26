use super::*;

/// The [Uniform distribution](https://en.wikipedia.org/wiki/Continuous_uniform_distribution) over floating point types.
///
/// # Implementation notes
///
/// Floating-point samplers do not distinguish between exclusive and inclusive bounds:
/// both constructors use the same scale-and-shift calculation with the bounds as given.
/// Equal and reversed bounds are accepted. Floating-point rounding and extreme values may affect the result.
///
/// Fast floating point values are requested directly from the `Rng` then scaled and shifted into the requested range.
///
/// The fallible constructors perform basic sanity checks and return [`UniformError::NonFinite`]
/// when the calculated bounds are not finite. The infallible constructors skip these checks.
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
