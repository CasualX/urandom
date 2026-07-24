use core::time::Duration;
use super::*;

const CHAR_GAP_START: u32 = 0xD800;
const CHAR_GAP_SIZE: u32 = 0xE000 - CHAR_GAP_START;

#[inline]
fn char_to_comp(value: char) -> u32 {
	let value = value as u32;
	if value >= CHAR_GAP_START + CHAR_GAP_SIZE {
		value - CHAR_GAP_SIZE
	}
	else {
		value
	}
}

#[inline]
fn char_from_comp(value: u32) -> char {
	let value = if value >= CHAR_GAP_START { value + CHAR_GAP_SIZE } else { value };
	char::from_u32(value).expect("UniformChar produced an invalid char")
}

/// The uniform distribution over a range of [`char`] values.
///
/// ```
/// let value = urandom::new().uniform('a'..='z');
/// assert!(value.is_ascii_lowercase());
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct UniformChar {
	inner: UniformInt<u32>,
}

impl SampleUniform for char {
	type Sampler = UniformChar;
}

impl UniformSampler<char> for UniformChar {
	#[inline]
	fn try_new(low: char, high: char) -> Result<Self, UniformError> {
		UniformInt::try_new(char_to_comp(low), char_to_comp(high)).map(|inner| UniformChar { inner })
	}

	#[inline]
	fn try_new_inclusive(low: char, high: char) -> Result<Self, UniformError> {
		UniformInt::try_new_inclusive(char_to_comp(low), char_to_comp(high)).map(|inner| UniformChar { inner })
	}
}

impl Distribution<char> for UniformChar {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> char {
		char_from_comp(self.inner.sample(rand))
	}
}

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
/// let value = urandom::new().uniform(Duration::from_secs(1)..Duration::from_secs(2));
/// assert!(value >= Duration::from_secs(1) && value < Duration::from_secs(2));
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniformDuration {
	base: u128,
	range: u128,
	threshold: u128,
}

impl UniformDuration {
	#[inline]
	fn from_base_range(base: u128, range: u128) -> UniformDuration {
		debug_assert_ne!(range, 0);
		let threshold = range.wrapping_neg() % range;
		UniformDuration { base, range, threshold }
	}
}

impl SampleUniform for Duration {
	type Sampler = UniformDuration;
}

impl UniformSampler<Duration> for UniformDuration {
	#[inline]
	fn try_new(low: Duration, high: Duration) -> Result<Self, UniformError> {
		if low >= high {
			return Err(UniformError::EmptyRange);
		}
		let base = duration_to_nanos(low);
		let range = duration_to_nanos(high) - base;
		Ok(Self::from_base_range(base, range))
	}

	#[inline]
	fn try_new_inclusive(low: Duration, high: Duration) -> Result<Self, UniformError> {
		if low > high {
			return Err(UniformError::EmptyRange);
		}
		let base = duration_to_nanos(low);
		let range = duration_to_nanos(high) - base + 1;
		Ok(Self::from_base_range(base, range))
	}
}

impl Distribution<Duration> for UniformDuration {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> Duration {
		loop {
			let value: u128 = rand.next();
			if value >= self.threshold {
				let offset = value % self.range;
				let value = self.base.checked_add(offset).expect("UniformDuration overflow");
				break duration_from_nanos(value);
			}
		}
	}
}
