use core::time::Duration;
use super::*;

const NANOS_PER_SEC: u128 = 1_000_000_000;

#[inline]
fn duration_to_nanos(value: Duration) -> u128 {
	value.as_secs() as u128 * NANOS_PER_SEC + value.subsec_nanos() as u128
}

#[inline]
fn duration_from_nanos(value: u128) -> Duration {
	let secs = u64::try_from(value / NANOS_PER_SEC).expect("UniformDuration produced an invalid Duration");
	Duration::new(secs, (value % NANOS_PER_SEC) as u32)
}

/// The uniform distribution over a range of [`Duration`] values.
///
/// Samples are uniform over all nanosecond-resolution values in the configured range.
///
/// ```
/// use core::time::Duration;
///
#[cfg_attr(feature = "getrandom", doc = "let value = urandom::new().uniform(Duration::from_secs(1)..Duration::from_secs(2));")]
#[cfg_attr(not(feature = "getrandom"), doc = "let value = urandom::seeded(42).uniform(Duration::from_secs(1)..Duration::from_secs(2));")]
/// assert!(value >= Duration::from_secs(1) && value < Duration::from_secs(2));
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct UniformDuration {
	inner: UniformInt<u128>,
}

impl SampleUniform for Duration {
	type Sampler = UniformDuration;
}

impl UniformSampler<Duration> for UniformDuration {
	#[inline]
	fn try_new(low: Duration, high: Duration) -> Result<Self, UniformError> {
		UniformInt::try_new(duration_to_nanos(low), duration_to_nanos(high)).map(|inner| UniformDuration { inner })
	}

	#[inline]
	fn try_new_inclusive(low: Duration, high: Duration) -> Result<Self, UniformError> {
		UniformInt::try_new_inclusive(duration_to_nanos(low), duration_to_nanos(high)).map(|inner| UniformDuration { inner })
	}
}

impl Distribution<Duration> for UniformDuration {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> Duration {
		duration_from_nanos(self.inner.sample(rand))
	}
}
