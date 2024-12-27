use super::*;

/// The [Standard exponential distribution](https://en.wikipedia.org/wiki/Exponential_distribution) `Exp(1)`.
///
/// This is equivalent to `Exp::new(1.0)` or sampling with `-rand.next().ln()`, but faster.
///
/// See [`Exp`] for the general exponential distribution.
///
/// # Plot
///
/// The following plot illustrates the exponential distribution with `λ = 1`.
///
/// ![Exponential distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/exponential_exp1.svg)
///
/// # Examples
///
/// ```
/// use urandom::distr::Exp1;
///
/// let value: f64 = urandom::new().sample(&Exp1);
/// println!("{value}");
/// ```
///
/// # Notes
///
/// Implemented via the ZIGNOR variant[^1] of the Ziggurat method. The exact description in the paper was adjusted to use tables for the exponential distribution rather than normal.
///
/// [^1]: Jurgen A. Doornik (2005). [*An Improved Ziggurat Method to Generate Normal Random Samples*](https://www.doornik.com/research/ziggurat.pdf). Nuffield College, Oxford
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exp1;

impl Distribution<f32> for Exp1 {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> f32 {
		// TODO: use optimal 32-bit implementation
		let x: f64 = self.sample(rand);
		x as f32
	}
}

// This could be done via `-rand.next::<f64>().ln()` but that is slower.
impl Distribution<f64> for Exp1 {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> f64 {
		#[inline]
		fn pdf(x: f64) -> f64 {
			(-x).exp()
		}

		#[inline]
		fn zero_case<R: Rng + ?Sized>(rand: &mut Random<R>, _u: f64) -> f64 {
			ziggurat::ZIG_EXP_R - rand.float01().ln()
		}

		ziggurat::ziggurat(
			rand,
			false,
			&ziggurat::ZIG_EXP_X,
			&ziggurat::ZIG_EXP_F,
			pdf,
			zero_case,
		)
	}
}

/// Error type returned from [`Exp`] constructors.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExpError {
	/// `lambda < 0` or `nan`.
	LambdaTooSmall,
}

impl fmt::Display for ExpError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			ExpError::LambdaTooSmall => "lambda is negative or NaN in exponential distribution",
		})
	}
}

#[cfg(feature = "std")]
impl std::error::Error for ExpError {}

pub trait ExpImpl<Float>: Sized {
	fn try_new(lambda: Float) -> Result<Self, ExpError>;
}

/// The [Exponential distribution](https://en.wikipedia.org/wiki/Exponential_distribution) `Exp(λ)`.
///
/// The exponential distribution is a continuous probability distribution with rate parameter `λ` (`lambda`).
/// It describes the time between events in a [Poisson](https://en.wikipedia.org/wiki/Poisson_distribution) process,
/// i.e. a process in which events occur continuously and independently at a constant average rate.
///
/// See [`Exp1`] for an optimised implementation for `λ = 1`.
///
/// # Density function
///
/// `f(x) = λ * exp(-λ * x)` for `x > 0`, when `λ > 0`.
///
/// For `λ = 0`, all samples yield infinity (because a Poisson process with rate 0 has no events).
///
/// # Plot
///
/// The following plot illustrates the exponential distribution with various values of `λ`.
/// The `λ` parameter controls the rate of decay as `x` approaches infinity, and the mean of the distribution is `1/λ`.
///
/// ![Exponential distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/exponential.svg)
///
/// # Examples
///
/// ```
/// use urandom::distr::Exp;
///
/// let exp = Exp::new(2.0);
/// let v = urandom::new().sample(&exp);
/// println!("{v} is from a Exp(2) distribution");
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exp<Float> {
	/// `lambda` stored as `1.0 / lambda`, since this is what we scale by.
	lambda_inverse: Float,
}

impl<Float> Exp<Float> where Self: ExpImpl<Float> {
	/// Constructs a new `Exp` with the given shape parameter `lambda`.
	#[inline]
	pub fn try_new(lambda: Float) -> Result<Exp<Float>, ExpError> {
		ExpImpl::try_new(lambda)
	}
	/// Constructs a new `Exp` with the given shape parameter `lambda`.
	#[track_caller]
	#[inline]
	pub fn new(lambda: Float) -> Exp<Float> {
		ExpImpl::try_new(lambda).unwrap()
	}
}

macro_rules! impl_exp {
	($f:ty) => {
		impl ExpImpl<$f> for Exp<$f> {
			#[inline]
			fn try_new(lambda: $f) -> Result<Self, ExpError> {
				if !(lambda >= 0.0) {
					return Err(ExpError::LambdaTooSmall);
				}
				Ok(Exp {
					lambda_inverse: 1.0 / lambda,
				})
			}
		}

		impl Distribution<$f> for Exp<$f> {
			#[inline]
			fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> $f {
				let x: $f = Exp1.sample(rand);
				x * self.lambda_inverse
			}
		}
	};
}

impl_exp!(f32);
impl_exp!(f64);

#[cfg(test)]
mod tests;
