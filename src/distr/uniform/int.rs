use super::*;

#[inline]
fn wmul32(a: u32, b: u32) -> (u32, u32) {
	let full = a as u64 * b as u64;
	let msw = (full >> 32) as u32;
	let lsw = (full & 0xffffffff) as u32;
	(msw, lsw)
}
#[inline]
fn wmul64(a: u64, b: u64) -> (u64, u64) {
	let full = a as u128 * b as u128;
	let msw = (full >> 64) as u64;
	let lsw = (full & 0xffffffffffffffff) as u64;
	(msw, lsw)
}

/// The [Discrete uniform distribution](https://en.wikipedia.org/wiki/Discrete_uniform_distribution) over integer types.
///
/// # Implementation notes
///
/// For simplicity, we use the same generic struct `UniformInt<T>` for all integer types `T`.
/// This gives us only one field type, `T`; to store unsigned values of this size, we take use of the fact that these conversions are no-ops.
///
/// For a closed range, the number of possible numbers we should generate is `range = (high - low + 1)`.
/// To avoid bias, we must ensure that the size of our sample space, is a multiple of `range`;
/// other values must be rejected (by replacing with a new random sample)[^1].
///
/// For more information on this bias see the `examples/int_bias.rs` example.
///
/// As a special case, we use `range = 0` to represent the full range of the result type (i.e. for the full inclusive range).
///
/// [^1]: Daniel Lemire (2018). [*Fast Random Integer Generation in an Interval*](https://arxiv.org/abs/1805.10941). Université du Québec (TELUQ), Canada
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniformInt<T> {
	base: T,
	// When T is signed, it is really an unsigned integer of the same size
	range: T,
}

impl<T> UniformInt<T> {
	pub(crate) const fn constant(base: T, range: T) -> UniformInt<T> {
		UniformInt { base, range }
	}
}

macro_rules! impl_uniform_int {
	($ty:ty, $unsigned:ty, $large:ty, $method:ident, $wmul:ident) => {
		impl SampleUniform for $ty {
			type Sampler = UniformInt<$ty>;
		}

		impl UniformSampler<$ty> for UniformInt<$ty> {
			#[inline]
			fn try_new(low: $ty, high: $ty) -> Result<UniformInt<$ty>, UniformError> {
				if low >= high {
					return Err(UniformError::EmptyRange);
				}
				// `high - low` may overflow for signed integers
				let range = high.wrapping_sub(low) as $unsigned as $ty;
				Ok(UniformInt { base: low, range })
			}

			#[inline]
			fn try_new_inclusive(low: $ty, high: $ty) -> Result<UniformInt<$ty>, UniformError> {
				if low > high {
					return Err(UniformError::EmptyRange);
				}
				// `high - low` may overflow for signed integers
				let range = high.wrapping_sub(low).wrapping_add(1) as $unsigned as $ty;
				Ok(UniformInt { base: low, range })
			}
		}

		impl Distribution<$ty> for UniformInt<$ty> {
			#[inline]
			fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> $ty {
				let range = self.range as $unsigned as $large;
				let mut zone = range;
				loop {
					let value = rand.$method();
					if range == 0 {
						break value as $ty;
					}
					let (msw, lsw) = $wmul(value, range);
					if lsw >= zone {
						break self.base.wrapping_add(msw as $ty);
					}
					if zone == range {
						zone = <$large>::wrapping_sub(0, range) % range;
						if lsw >= zone {
							break self.base.wrapping_add(msw as $ty);
						}
					}
				}
			}
		}
	};
}

impl_uniform_int! { i8, u8, u32, next_u32, wmul32 }
impl_uniform_int! { u8, u8, u32, next_u32, wmul32 }

impl_uniform_int! { i16, u16, u32, next_u32, wmul32 }
impl_uniform_int! { u16, u16, u32, next_u32, wmul32 }

impl_uniform_int! { i32, u32, u64, next_u64, wmul64 }
impl_uniform_int! { u32, u32, u64, next_u64, wmul64 }

impl_uniform_int! { i64, u64, u64, next_u64, wmul64 }
impl_uniform_int! { u64, u64, u64, next_u64, wmul64 }

// Interestingly make usize/isize use the same code paths
// This keeps the result deterministic regardless of pointer width
#[cfg(target_pointer_width = "32")]
impl_uniform_int! { isize, u32, u64, next_u64, wmul64 }
#[cfg(target_pointer_width = "32")]
impl_uniform_int! { usize, u32, u64, next_u64, wmul64 }

#[cfg(target_pointer_width = "64")]
impl_uniform_int! { isize, u64, u64, next_u64, wmul64 }
#[cfg(target_pointer_width = "64")]
impl_uniform_int! { usize, u64, u64, next_u64, wmul64 }
