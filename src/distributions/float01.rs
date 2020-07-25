use crate::{Distribution, Rng, Random};

/// A distribution to sample floating point numbers uniformly in the open interval `(0, 1)`, i.e. not including either endpoint.
///
/// # Precision
///
/// This implementation does not suffer from bias in the low bits of the mantissa.
///
/// # Implementation notes
///
/// The implementation is simple, fast and straighforward from the following observations:
///
/// Dividing a floating point number in half is exactly the same as subtracting one from its exponent.
/// With a given exponent the mantissa provides all values between 2<sup>exp</sup> and 2<sup>exp+1</sup>.
///
/// Eg. With an exponent of `-1` the resulting mantissa defines a floating point number in half-open interval `[0.5, 1.0)`.
/// With an exponent of `-2` the floating point number is in the half-open interval `[0.25, 0.5)` and so on.
///
/// In a loop flip a coin, if heads produce a floating point number with the current exponent starting at `-1` if tails subtract one from the exponent and repeat.
/// This produces smaller floating point numbers with exponentially less probability (of base 2) which is exactly what we want.
///
/// The loop can be avoided by generating a single `u64` and looking at the individual bits, subtract one for every `0` bit until a `1` bit is encountered.
/// This operation is efficiently implemented in hardware known as the _count leading zeros_ instruction (eg. [`LZCNT` in x86](https://www.felixcloutier.com/x86/lzcnt)).
///
/// There is a small bias in case the Rng outputs all zeros but in practice this should never happen unless your PRNG is broken.
///
/// The result is two calls to the Rng, one for generating 64 bits worth of coin flips and one for generating the mantissa of the resulting float.
#[derive(Copy, Clone, Debug)]
pub struct Float01;

impl Distribution<f32> for Float01 {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> f32 {
		let exp = 0b0_01111110 - rng.next_u64().leading_zeros();
		let mantissa = crate::impls::mantissa_f32(rng.next_f32());
		f32::from_bits(exp << (f32::MANTISSA_DIGITS - 1) | mantissa)
	}
}
impl Distribution<f64> for Float01 {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> f64 {
		let exp = (0b0_01111111110 - rng.next_u64().leading_zeros()) as u64;
		let mantissa = crate::impls::mantissa_f64(rng.next_f64());
		f64::from_bits(exp << (f64::MANTISSA_DIGITS - 1) | mantissa)
	}
}

#[test]
fn test_yolo() {
	for float in crate::new().samples(Float01).take(1000) {
		let float: f32 = float;
		assert!(float > 0.0 && float < 1.0, "float({}) bits({:#x})", float, float.to_bits());
	}
}

#[test]
fn test_edges() {
	let mut rng = crate::rng::MockRng::slice(&[0, 0, !0, !0]);
	let low: f64 = rng.sample(&Float01);
	let high: f64 = rng.sample(&Float01);
	assert!(low > 0.0 && low < 1.0, "double({}) bits({:#x})", low, low.to_bits());
	assert!(high > 0.0 && high < 1.0, "double({}) bits({:#x})", high, high.to_bits());
}
