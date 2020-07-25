use core::fmt;
use crate::{Distribution, Random, Rng};
use crate::distributions::{SampleUniform, UniformSampler};

trait WideMul: Sized {
	fn wmul(self, other: Self) -> (Self, Self);
}
impl WideMul for u32 {
	#[inline]
	fn wmul(self, other: u32) -> (u32, u32) {
		let full = self as u64 * other as u64;
		let msw = (full >> 32) as u32;
		let lsw = (full & 0xffffffff) as u32;
		(msw, lsw)
	}
}
impl WideMul for u64 {
	#[inline]
	fn wmul(self, other: u64) -> (u64, u64) {
		let full = self as u128 * other as u128;
		let msw = (full >> 64) as u64;
		let lsw = (full & 0xffffffffffffffff) as u64;
		(msw, lsw)
	}
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
///
/// As a special case, we use `range = 0` to represent the full range of the result type (i.e. for the full inclusive range).
///
/// For more information on this bias see the `examples/int_bias.rs` example.
#[derive(Copy, Clone, Debug)]
pub struct UniformInt<T> {
	base: T,
	// When T is signed, range and reject are really unsigned integers of the same size
	range: T,
	// Number of values to reject from the underlying `Rng` which always generates a uniform random 32-bit or 64-bit integer.
	reject: T,
}

impl<T> UniformInt<T> {
	pub(crate) const fn constant(base: T, range: T, reject: T) -> UniformInt<T> {
		UniformInt { base, range, reject }
	}
}

#[inline]
fn ints_to_reject32(range: u32) -> u32 {
	// Small table with precalculated reject values
	// See the `examples/int_bias.rs` to calculate them
	const REJECT32: [u8; 64] = [
		0, 0, 1, 0, 1, 4, 4, 0,
		4, 6, 4, 4, 9, 4, 1, 0,
		1, 4, 6, 16, 4, 4, 12, 16,
		21, 22, 22, 4, 16, 16, 4, 0,
		4, 18, 11, 4, 7, 6, 22, 16,
		37, 4, 16, 4, 31, 12, 42, 16,
		39, 46, 1, 48, 42, 22, 26, 32,
		25, 16, 51, 16, 57, 4, 4, 0,
	];
	if range == 0 {
		0
	}
	else if range as usize <= REJECT32.len() {
		REJECT32[(range - 1) as usize] as u32
	}
	else {
		u32::wrapping_sub(0, range) % range
	}
}
#[inline]
fn ints_to_reject64(range: u64) -> u64 {
	// Small table with precalculated reject values
	// See the `examples/int_bias.rs` to calculate them
	const REJECT64: [u8; 64] = [
		0, 0, 1, 0, 1, 4, 2, 0,
		7, 6, 5, 4, 3, 2, 1, 0,
		1, 16, 17, 16, 16, 16, 6, 16,
		16, 16, 25, 16, 24, 16, 16, 0,
		16, 18, 16, 16, 12, 36, 16, 16,
		16, 16, 41, 16, 16, 6, 25, 16,
		2, 16, 1, 16, 15, 52, 16, 16,
		55, 24, 5, 16, 16, 16, 16, 0,
	];
	if range == 0 {
		0
	}
	else if range <= REJECT64.len() as u64 {
		REJECT64[(range - 1) as usize] as u64
	}
	else {
		u64::wrapping_sub(0, range) % range
	}
}
macro_rules! ints_to_reject {
	(next_u32) => { ints_to_reject32 };
	(next_u64) => { ints_to_reject64 };
}

macro_rules! impl_uniform_int {
	($ty:ty, $unsigned:ty, $large:ty, $method:ident) => {
		impl SampleUniform for $ty {
			type Sampler = UniformInt<$ty>;
		}
		impl UniformSampler<$ty> for UniformInt<$ty> {
			#[inline]
			fn new(low: $ty, high: $ty) -> UniformInt<$ty> {
				if low >= high {
					uniform_int_new_error(low, high);
				}
				Self::new_inclusive(low, high - 1)
			}
			#[inline]
			fn new_inclusive(low: $ty, high: $ty) -> UniformInt<$ty> {
				if low > high {
					uniform_int_new_inclusive_error(low, high);
				}
				// `high - low` may overflow for signed integers
				let range = high.wrapping_sub(low).wrapping_add(1) as $unsigned;
				let reject = ints_to_reject!($method)(range as $large) as $ty;
				let base = low;
				let range = range as $ty;
				UniformInt { base, range, reject }
			}
		}
		impl Distribution<$ty> for UniformInt<$ty> {
			#[inline]
			fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> $ty {
				let range = self.range as $unsigned as $large;
				let zone = self.reject as $unsigned as $large;
				loop {
					let v = rng.$method();
					if range == 0 {
						break v as $ty;
					}
					let (msw, lsw) = v.wmul(range);
					if lsw >= zone {
						break self.base.wrapping_add(msw as $ty);
					}
				}
			}
		}
	};
}

impl_uniform_int! { i8, u8, u32, next_u32 }
impl_uniform_int! { u8, u8, u32, next_u32 }

impl_uniform_int! { i16, u16, u32, next_u32 }
impl_uniform_int! { u16, u16, u32, next_u32 }

impl_uniform_int! { i32, u32, u32, next_u32 }
impl_uniform_int! { u32, u32, u32, next_u32 }

impl_uniform_int! { i64, u64, u64, next_u64 }
impl_uniform_int! { u64, u64, u64, next_u64 }

// Interestingly make usize/isize use the same code paths
// This keeps the result deterministic regardless of pointer width
#[cfg(target_pointer_width = "32")]
impl_uniform_int! { isize, u32, u64, next_u64 }
#[cfg(target_pointer_width = "32")]
impl_uniform_int! { usize, u32, u64, next_u64 }

#[cfg(target_pointer_width = "64")]
impl_uniform_int! { isize, u64, u64, next_u64 }
#[cfg(target_pointer_width = "64")]
impl_uniform_int! { usize, u64, u64, next_u64 }

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
fn test_edges() {
	let distr = UniformInt::new_inclusive(i32::MIN, i32::MAX);
	println!("distr: {:#x?}", distr);
	let mut rng = crate::new();
	for _ in 0..10000 {
		let _value = rng.sample(&distr);
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
