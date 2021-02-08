use core::fmt;
use crate::{Distribution, Random, Rng};
use crate::distributions::{SampleUniform, UniformSampler};

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

/// Uniform distribution over integral types.
///
/// # Implementation notes
///
/// For simplicity, we use the same generic struct `UniformInt<T>` for all integer types `T`.
/// This gives us only one field type, `T`; to store unsigned values of this size, we take use the fact that these conversions are no-ops.
///
/// For a closed range, the number of possible numbers we should generate is `range = (high - low + 1)`.
/// To avoid bias, we must ensure that the size of our sample space, is a multiple of `range`;
/// other values must be rejected (by replacing with a new random sample).
/// See [Fast Random Integer Generation in an Interval](https://arxiv.org/abs/1805.10941) for the algorithm used.
///
/// As a special case, we use `range = 0` to represent the full range of the result type (i.e. for the full inclusive range).
///
/// For more information on this bias see the `examples/int_bias.rs` example.
#[derive(Copy, Clone, Debug)]
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
			fn new(low: $ty, high: $ty) -> UniformInt<$ty> {
				if low >= high {
					uniform_int_new_error(low, high);
				}
				// `high - low` may overflow for signed integers
				let range = high.wrapping_sub(low) as $unsigned as $ty;
				UniformInt { base: low, range }
			}
			#[inline]
			fn new_inclusive(low: $ty, high: $ty) -> UniformInt<$ty> {
				if low > high {
					uniform_int_new_inclusive_error(low, high);
				}
				// `high - low` may overflow for signed integers
				let range = high.wrapping_sub(low).wrapping_add(1) as $unsigned as $ty;
				UniformInt { base: low, range }
			}
		}
		impl Distribution<$ty> for UniformInt<$ty> {
			#[inline]
			fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> $ty {
				let range = self.range as $unsigned as $large;
				let mut zone = range;
				loop {
					let v = rng.$method();
					if range == 0 {
						break v as $ty;
					}
					let (msw, lsw) = $wmul(v, range);
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

#[cold]
fn uniform_int_new_error<T: fmt::Debug>(low: T, high: T) -> ! {
	panic!("UniformSampler::new called with `low >= high` where low: {:?} and high: {:?}", low, high);
}

#[cold]
fn uniform_int_new_inclusive_error<T: fmt::Debug>(low: T, high: T) -> ! {
	panic!("UniformSampler::new_inclusive called with `low > high` where low: {:?} and high: {:?}", low, high);
}

//----------------------------------------------------------------

#[test]
fn test_bias() {
	let distr = UniformInt::new_inclusive(0u32, 0xC0000000);
	println!("distr: {:#x?}", distr);

	let mut rng = crate::new();
	let mut buckets = [0u32; 3];

	for _ in 0..10000 {
		let value = rng.sample(&distr);

		if value < 0x40000000 {
			buckets[0] += 1;
		}
		else if value < 0x80000000 {
			buckets[1] += 1;
		}
		else if value <= 0xC0000000 {
			buckets[2] += 1;
		}
		else {
			panic!("value: {:#x}", value);
		}
	}

	let mean = (buckets[0] as i64 + buckets[1] as i64 + buckets[2] as i64) / 3;
	let pass = buckets.iter().all(|&odd| (odd as i64 - mean).abs() < 1000);
	println!("mean:{} buckets:{:?} pass:{}", mean, buckets, pass);
	assert!(pass);
}

#[test]
fn test_edges_large() {
	let distr = UniformInt::new_inclusive(u32::MIN, u32::MAX);
	println!("distr: {:#x?}", distr);
	let mut rng = crate::new();
	let mut zeros = 0;
	for _ in 0..10000 {
		let value = rng.sample(&distr);
		if value == 0 {
			zeros += 1;
		}
	}
	assert!(zeros < 5, "found {} zero samples!", zeros);
}

#[test]
fn test_edges_small() {
	let distr1 = UniformInt::new_inclusive(10, 10);
	let distr2 = UniformInt::new(23, 24);
	let mut rng = crate::new();
	for _ in 0..100 {
		let value1 = rng.sample(&distr1);
		let value2 = rng.sample(&distr2);
		assert_eq!(value1, 10);
		assert_eq!(value2, 23);
	}
}

#[test]
fn test_yolo() {
	let mut rng = crate::new();
	for _ in 0..10000 {
		let mut low: i16 = rng.next();
		let mut high: i16 = rng.next();
		if high < low {
			let tmp = low;
			low = high;
			high = tmp;
		}
		let value = rng.range(low..=high);
		assert!(value >= low && value <= high);
		if low != high {
			let value = rng.range(low..high);
			assert!(value >= low && value < high);
		}
	}
}
