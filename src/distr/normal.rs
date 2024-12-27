use super::*;

/// The [Standard normal distribution](https://en.wikipedia.org/wiki/Normal_distribution#Standard_normal_distribution) `N(0, 1)`.
///
/// This is equivalent to `Normal::new(0.0, 1.0)`, but faster.
///
/// See [`Normal`] for the general normal distribution.
///
/// # Plot
///
/// The following diagram shows the standard normal distribution.
///
/// ![Standard normal distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/standard_normal.svg)
///
/// # Examples
///
/// ```
/// use urandom::distr::StandardNormal;
///
/// let value: f64 = urandom::new().sample(&StandardNormal);
/// println!("{value}");
/// ```
///
/// # Notes
///
/// Implemented via the ZIGNOR variant[^1] of the Ziggurat method.
///
/// [^1]: Jurgen A. Doornik (2005). [*An Improved Ziggurat Method to Generate Normal Random Samples*](https://www.doornik.com/research/ziggurat.pdf). Nuffield College, Oxford
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StandardNormal;

impl Distribution<f32> for StandardNormal {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> f32 {
		let x: f64 = self.sample(rand);
		x as f32
	}
}

impl Distribution<f64> for StandardNormal {
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> f64 {
		#[inline]
		fn pdf(x: f64) -> f64 {
			(-x * x / 2.0).exp()
		}

		#[inline]
		fn zero_case<R: Rng + ?Sized>(rand: &mut Random<R>, u: f64) -> f64 {
			// compute a random number in the tail by hand

			// strange initial conditions, because the loop is not
			// do-while, so the condition should be true on the first
			// run, they get overwritten anyway (0 < 1, so these are
			// good).
			let mut x = 1.0f64;
			let mut y = 0.0f64;

			while -2.0 * y < x * x {
				let x_: f64 = rand.float01();
				let y_: f64 = rand.float01();

				x = x_.ln() / ziggurat::ZIG_NORM_R;
				y = y_.ln();
			}

			if u < 0.0 {
				x - ziggurat::ZIG_NORM_R
			}
			else {
				ziggurat::ZIG_NORM_R - x
			}
		}

		ziggurat::ziggurat(
			rand,
			true, // this is symmetric
			&ziggurat::ZIG_NORM_X,
			&ziggurat::ZIG_NORM_F,
			pdf,
			zero_case,
		)
	}
}

/// Error type returned from [`Normal`] and [`LogNormal`] constructors.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NormalError {
	/// The mean value is too small (log-normal samples must be positive).
	MeanTooSmall,
	/// The standard deviation or other dispersion parameter is not finite.
	BadVariance,
}

impl fmt::Display for NormalError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			NormalError::MeanTooSmall => "mean < 0 or NaN in log-normal distribution",
			NormalError::BadVariance => "variation parameter is non-finite in (log)normal distribution",
		})
	}
}

#[cfg(feature = "std")]
impl std::error::Error for NormalError {}

pub trait NormalImpl<Float>: Sized {
	fn try_new(_1: Float, _2: Float) -> Result<Self, NormalError>;
	fn try_from_mean_cv(_1: Float, _2: Float) -> Result<Self, NormalError>;
	fn from_zscore(&self, zscore: Float) -> Float;
}

/// The [Normal distribution](https://en.wikipedia.org/wiki/Normal_distribution) `N(μ, σ²)`.
///
/// The normal distribution, also known as the Gaussian distribution or bell curve,
/// is a continuous probability distribution with mean `μ` (`mu`) and standard deviation `σ` (`sigma`).
/// It is used to model continuous data that tend to cluster around a mean.
/// The normal distribution is symmetric and characterized by its bell-shaped curve.
///
/// See [`StandardNormal`] for an optimised implementation for `μ = 0` and `σ = 1`.
///
/// # Density function
///
/// `f(x) = (1 / sqrt(2π σ²)) * exp(-((x - μ)² / (2σ²)))`
///
/// # Plot
///
/// The following diagram shows the normal distribution with various values of `μ` and `σ`.
/// The blue curve is the [`StandardNormal`] distribution, `N(0, 1)`.
///
/// ![Normal distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/normal.svg)
///
/// # Examples
///
/// ```
/// use urandom::distr::Normal;
///
/// // mean 2, standard deviation 3
/// let normal = Normal::new(2.0, 3.0);
/// let v = urandom::new().sample(&normal);
/// println!("{v} is from a N(2, 9) distribution");
/// ```
///
/// # Notes
///
/// Implemented via the ZIGNOR variant[^1] of the Ziggurat method.
///
/// [^1]: Jurgen A. Doornik (2005). [*An Improved Ziggurat Method to Generate Normal Random Samples*](https://www.doornik.com/research/ziggurat.pdf). Nuffield College, Oxford
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Normal<Float> {
	mean: Float,
	std_dev: Float,
}

impl<Float: Copy> Normal<Float> where Self: NormalImpl<Float> {
	/// Constructs, from mean and standard deviation.
	///
	/// Parameters:
	///
	/// - mean (`μ`, unrestricted)
	/// - standard deviation (`σ`, must be finite)
	#[inline]
	pub fn try_new(mean: Float, std_dev: Float) -> Result<Normal<Float>, NormalError> {
		NormalImpl::try_new(mean, std_dev)
	}
	/// Constructs, from mean and standard deviation.
	///
	/// Parameters:
	///
	/// - mean (`μ`, unrestricted)
	/// - standard deviation (`σ`, must be finite)
	#[track_caller]
	#[inline]
	pub fn new(mean: Float, std_dev: Float) -> Normal<Float> {
		NormalImpl::try_new(mean, std_dev).unwrap()
	}

	/// Constructs, from mean and coefficient of variation.
	///
	/// Parameters:
	///
	/// - mean (`μ`, unrestricted)
	/// - coefficient of variation (`cv = abs(σ / μ)`)
	#[inline]
	pub fn try_from_mean_cv(mean: Float, cv: Float) -> Result<Normal<Float>, NormalError> {
		NormalImpl::try_from_mean_cv(mean, cv)
	}
	/// Constructs, from mean and coefficient of variation.
	///
	/// Parameters:
	///
	/// - mean (`μ`, unrestricted)
	/// - coefficient of variation (`cv = abs(σ / μ)`)
	#[track_caller]
	#[inline]
	pub fn from_mean_cv(mean: Float, cv: Float) -> Normal<Float> {
		NormalImpl::try_from_mean_cv(mean, cv).unwrap()
	}

	/// Returns the mean (`μ`) of the distribution.
	#[inline]
	pub fn mean(&self) -> Float {
		self.mean
	}

	/// Returns the standard deviation (`σ`) of the distribution.
	#[inline]
	pub fn std_dev(&self) -> Float {
		self.std_dev
	}

	/// Sample from a z-score.
	///
	/// This may be useful for generating correlated samples `x1` and `x2` from two different distributions, as follows.
	///
	/// ```
	/// # use urandom::distr::*;
	/// let mut rand = urandom::new();
	/// let z = rand.sample(&StandardNormal);
	/// let x1 = Normal::new(0.0, 1.0).from_zscore(z);
	/// let x2 = Normal::new(2.0, -3.0).from_zscore(z);
	/// ```
	#[inline]
	pub fn from_zscore(&self, zscore: Float) -> Float {
		NormalImpl::from_zscore(self, zscore)
	}
}

macro_rules! impl_normal {
	($ty:ty) => {
		impl NormalImpl<$ty> for Normal<$ty> {
			#[inline]
			fn try_new(mean: $ty, std_dev: $ty) -> Result<Normal<$ty>, NormalError> {
				if !std_dev.is_finite() {
					return Err(NormalError::BadVariance);
				}
				Ok(Normal { mean, std_dev })
			}

			#[inline]
			fn try_from_mean_cv(mean: $ty, cv: $ty) -> Result<Normal<$ty>, NormalError> {
				if !cv.is_finite() || cv < 0.0 {
					return Err(NormalError::BadVariance);
				}
				let std_dev = cv * mean;
				Ok(Normal { mean, std_dev })
			}

			#[inline]
			fn from_zscore(&self, zscore: $ty) -> $ty {
				self.std_dev.mul_add(zscore, self.mean)
			}
		}

		impl Distribution<$ty> for Normal<$ty> {
			fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> $ty {
				self.from_zscore(StandardNormal.sample(rand))
			}
		}
	}
}

impl_normal!(f32);
impl_normal!(f64);

/// The [Log-normal distribution](https://en.wikipedia.org/wiki/Log-normal_distribution) `ln N(μ, σ²)`.
///
/// This is the distribution of the random variable `X = exp(Y)` where `Y` is normally distributed with mean `μ` and variance `σ²`.
/// In other words, if `X` is log-normal distributed, then `ln(X)` is `N(μ, σ²)` distributed.
///
/// # Plot
///
/// The following diagram shows the log-normal distribution with various values of `μ` and `σ`.
///
/// ![Log-normal distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/log_normal.svg)
///
/// # Examples
///
/// ```
/// use urandom::distr::LogNormal;
///
/// // mean 2, standard deviation 3
/// let log_normal = LogNormal::new(2.0, 3.0);
/// let v = urandom::new().sample(&log_normal);
/// println!("{v} is from an ln N(2, 9) distribution");
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogNormal<Float> {
	norm: Normal<Float>,
}

impl<Float: Copy> LogNormal<Float> where Self: NormalImpl<Float> {
	/// Constructs, from (log-space) mean and standard deviation.
	///
	/// Parameters are the "standard" log-space measures (these are the mean and standard deviation of the logarithm of samples):
	///
	/// - `mu` (`μ`, unrestricted) is the mean of the underlying distribution
	/// - `sigma` (`σ`, must be finite) is the standard deviation of the underlying normal distribution
	#[inline]
	pub fn try_new(mu: Float, sigma: Float) -> Result<LogNormal<Float>, NormalError> {
		NormalImpl::try_new(mu, sigma)
	}
	/// Constructs, from (log-space) mean and standard deviation.
	///
	/// Parameters are the "standard" log-space measures (these are the mean and standard deviation of the logarithm of samples):
	///
	/// - `mu` (`μ`, unrestricted) is the mean of the underlying distribution
	/// - `sigma` (`σ`, must be finite) is the standard deviation of the underlying normal distribution
	#[track_caller]
	#[inline]
	pub fn new(mu: Float, sigma: Float) -> LogNormal<Float> {
		NormalImpl::try_new(mu, sigma).unwrap()
	}

	/// Constructs, from (linear-space) mean and coefficient of variation.
	///
	/// Parameters are linear-space measures:
	///
	/// - mean (`μ > 0`) is the (real) mean of the distribution
	/// - coefficient of variation (`cv = σ / μ`, requiring `cv ≥ 0`) is a standardized measure of dispersion
	///
	/// As a special exception, `μ = 0, cv = 0` is allowed (samples are `-inf`).
	#[inline]
	pub fn try_from_mean_cv(mean: Float, cv: Float) -> Result<LogNormal<Float>, NormalError> {
		NormalImpl::try_from_mean_cv(mean, cv)
	}
	/// Constructs, from (linear-space) mean and coefficient of variation.
	///
	/// Parameters are linear-space measures:
	///
	/// - mean (`μ > 0`) is the (real) mean of the distribution
	/// - coefficient of variation (`cv = σ / μ`, requiring `cv ≥ 0`) is a standardized measure of dispersion
	///
	/// As a special exception, `μ = 0, cv = 0` is allowed (samples are `-inf`).
	#[track_caller]
	#[inline]
	pub fn from_mean_cv(mean: Float, cv: Float) -> LogNormal<Float> {
		NormalImpl::try_from_mean_cv(mean, cv).unwrap()
	}

	/// Sample from a z-score.
	///
	/// This may be useful for generating correlated samples `x1` and `x2` from two different distributions, as follows.
	///
	/// ```
	/// # use urandom::distr::{LogNormal, StandardNormal};
	/// let mut rand = urandom::new();
	/// let z = rand.sample(&StandardNormal);
	/// let x1 = LogNormal::from_mean_cv(3.0, 1.0).from_zscore(z);
	/// let x2 = LogNormal::from_mean_cv(2.0, 4.0).from_zscore(z);
	/// ```
	#[inline]
	pub fn from_zscore(&self, zscore: Float) -> Float {
		NormalImpl::from_zscore(self, zscore)
	}
}

macro_rules! impl_log_normal {
	($ty:ty) => {
		impl NormalImpl<$ty> for LogNormal<$ty> {
			#[inline]
			fn try_new(mu: $ty, sigma: $ty) -> Result<LogNormal<$ty>, NormalError> {
				let norm = Normal::try_new(mu, sigma)?;
				Ok(LogNormal { norm })
			}

			#[inline]
			fn try_from_mean_cv(mean: $ty, cv: $ty) -> Result<LogNormal<$ty>, NormalError> {
				if cv == 0.0 {
					let mu = mean.ln();
					let norm = Normal::try_new(mu, 0.0)?;
					return Ok(LogNormal { norm });
				}
				if !(mean > 0.0) {
					return Err(NormalError::MeanTooSmall);
				}
				if !(cv >= 0.0) {
					return Err(NormalError::BadVariance);
				}

				// Using X ~ lognormal(μ, σ), CV² = Var(X) / E(X)²
				// E(X) = exp(μ + σ² / 2) = exp(μ) × exp(σ² / 2)
				// Var(X) = exp(2μ + σ²)(exp(σ²) - 1) = E(X)² × (exp(σ²) - 1)
				// but Var(X) = (CV × E(X))² so CV² = exp(σ²) - 1
				// thus σ² = log(CV² + 1)
				// and exp(μ) = E(X) / exp(σ² / 2) = E(X) / sqrt(CV² + 1)
				let a = 1.0 + cv * cv; // e
				let mu = 0.5 * (mean * mean / a).ln();
				let sigma = a.ln().sqrt();
				let norm = Normal::try_new(mu, sigma)?;
				Ok(LogNormal { norm })
			}

			#[inline]
			fn from_zscore(&self, zscore: $ty) -> $ty {
				self.norm.from_zscore(zscore).exp()
			}
		}

		impl Distribution<$ty> for LogNormal<$ty> {
			fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> $ty {
				self.norm.sample(rand).exp()
			}
		}
	}
}

impl_log_normal!(f32);
impl_log_normal!(f64);

#[cfg(test)]
mod tests;
