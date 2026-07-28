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
