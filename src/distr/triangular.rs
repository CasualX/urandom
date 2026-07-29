use super::*;

/// Error type returned from [`Triangular`] constructors.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TriangularError {
	/// `max < min`, or either bound is NaN.
	RangeTooSmall,
	/// `mode < min`, `mode > max`, or `mode` is NaN.
	ModeRange,
}

impl fmt::Display for TriangularError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			TriangularError::RangeTooSmall => "requirement min <= max is not met in triangular distribution",
			TriangularError::ModeRange => "mode is outside [min, max] in triangular distribution",
		})
	}
}

#[cfg(feature = "std")]
impl std::error::Error for TriangularError {}

/// The [triangular distribution](https://en.wikipedia.org/wiki/Triangular_distribution) `Triangular(min, max, mode)`.
///
/// This is a continuous probability distribution over the inclusive range from `min` to `max`, with its greatest density at `mode`.
///
/// Equal bounds are accepted when the mode is equal to them, producing a degenerate distribution whose samples all equal that bound.
///
/// # Plot
///
/// The following plot illustrates the triangular distribution with various values of `min`, `max`, and `mode`.
///
/// ![Triangular distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/triangular.svg)
///
/// # Examples
///
/// ```
/// use urandom::distr::Triangular;
///
/// let triangular = Triangular::new(0.0, 5.0, 2.5);
#[cfg_attr(feature = "getrandom", doc = "let value = urandom::new().sample(&triangular);")]
#[cfg_attr(not(feature = "getrandom"), doc = "let value = urandom::seeded(42).sample(&triangular);")]
/// assert!((0.0..=5.0).contains(&value));
/// ```
///
/// # Sampling
///
/// Sampling uses the inverse cumulative distribution function and consumes one
/// [`StandardUniform`] floating-point sample.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Triangular<Float> {
	min: Float,
	max: Float,
	mode: Float,
}

// Sealed trait, not publicly exported
pub trait TriangularImpl<Float>: Sized {
	fn try_new(min: Float, max: Float, mode: Float) -> Result<Self, TriangularError>;
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> Float;
}

impl<Float: Copy> Triangular<Float> where Self: TriangularImpl<Float> {
	/// Tries to construct a triangular distribution with the given `min`, `max`, and `mode`.
	///
	/// The bounds must satisfy `min <= max`, and the mode must lie in the inclusive range `[min, max]`.
	#[inline]
	pub fn try_new(min: Float, max: Float, mode: Float) -> Result<Self, TriangularError> {
		TriangularImpl::try_new(min, max, mode)
	}

	/// Constructs a triangular distribution with the given `min`, `max`, and `mode`.
	///
	/// # Panics
	///
	/// Panics if `min > max`, either bound is NaN, or `mode` is not within the inclusive range `[min, max]`.
	#[track_caller]
	#[inline]
	pub fn new(min: Float, max: Float, mode: Float) -> Self {
		TriangularImpl::try_new(min, max, mode).unwrap()
	}

	/// Returns the lower bound of the distribution.
	#[inline]
	pub fn min(&self) -> Float {
		self.min
	}

	/// Returns the upper bound of the distribution.
	#[inline]
	pub fn max(&self) -> Float {
		self.max
	}

	/// Returns the mode (most likely value) of the distribution.
	#[inline]
	pub fn mode(&self) -> Float {
		self.mode
	}
}

impl<Float> Distribution<Float> for Triangular<Float> where Self: TriangularImpl<Float> {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> Float {
		TriangularImpl::sample(self, rand)
	}
}

macro_rules! impl_triangular {
	($f:ty) => {
		impl TriangularImpl<$f> for Triangular<$f> {
			#[inline]
			fn try_new(min: $f, max: $f, mode: $f) -> Result<Self, TriangularError> {
				if !(max >= min) {
					return Err(TriangularError::RangeTooSmall);
				}
				if !(mode >= min && max >= mode) {
					return Err(TriangularError::ModeRange);
				}
				Ok(Triangular { min, max, mode })
			}

			#[inline]
			fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> $f {
				let f: $f = StandardUniform.sample(rand);
				let diff_mode_min = self.mode - self.min;
				let range = self.max - self.min;
				let f_range = f * range;

				if f_range < diff_mode_min {
					self.min + (f_range * diff_mode_min).sqrt()
				}
				else {
					self.max - ((range - f_range) * (self.max - self.mode)).sqrt()
				}
			}
		}
	};
}

impl_triangular!(f32);
impl_triangular!(f64);

#[cfg(test)]
mod tests;
