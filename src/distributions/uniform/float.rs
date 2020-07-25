use crate::{Distribution, Random, Rng};
use crate::distributions::{SampleUniform, UniformSampler};

/// Uniform distribution over the floating point types.
///
/// # Implementation notes
///
/// Floating point types always include the lower bound and exclude the upper bound regardless of which constructor was chosen.
/// When the high argument is less than the low argument this is reversed and the upper bound is included and lower bound is excluded.
///
/// Fast floating point values are requested directly from the `Rng` then scaled and shifted into the requested range.
///
/// When the inputs are not finite or become non-finite during setup the result may produce unexpected results (eg. `NaN`).
#[derive(Copy, Clone, Debug)]
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
	fn new_inclusive(low: f32, high: f32) -> UniformFloat<f32> {
		Self::new(low, high)
	}
}
impl Distribution<f32> for UniformFloat<f32> {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> f32 {
		rng.next_f32() * self.scale + self.base
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
	fn new_inclusive(low: f64, high: f64) -> UniformFloat<f64> {
		Self::new(low, high)
	}
}
impl Distribution<f64> for UniformFloat<f64> {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> f64 {
		rng.next_f64() * self.scale + self.base
	}
}
