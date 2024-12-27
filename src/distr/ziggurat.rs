use crate::{Random, Rng};
pub use super::ziggurat_tables::*;

#[inline(always)]
fn into_float_with_exponent(bits: u64, exp: u64) -> f64 {
	f64::from_bits(bits >> 12 | (1023 + exp) << 52)
}

/// Sample a random number using the Ziggurat method (specifically the
/// ZIGNOR variant from Doornik 2005). Most of the arguments are
/// directly from the paper:
///
/// * `rand`: source of randomness
/// * `symmetric`: whether this is a symmetric distribution, or one-sided with P(x < 0) = 0.
/// * `X`: the $x_i$ abscissae.
/// * `F`: precomputed values of the PDF at the $x_i$, (i.e. $f(x_i)$)
/// * `F_DIFF`: precomputed values of $f(x_i) - f(x_{i+1})$
/// * `pdf`: the probability density function
/// * `zero_case`: manual sampling from the tail when we chose the
///	bottom box (i.e. i == 0)
#[inline(always)] // Forced inlining improves the perf by 25-50%
pub fn ziggurat<R: Rng + ?Sized, P, Z>(
	rand: &mut Random<R>,
	symmetric: bool,
	x_tab: ZigTable,
	f_tab: ZigTable,
	mut pdf: P,
	mut zero_case: Z,
) -> f64
where
	P: FnMut(f64) -> f64,
	Z: FnMut(&mut Random<R>, f64) -> f64,
{
	loop {
		// As an optimisation we re-implement the conversion to a f64.
		// From the remaining 12 most significant bits we use 8 to construct `i`.
		// This saves us generating a whole extra random number, while the added
		// precision of using 64 bits for f64 does not buy us much.
		let bits = rand.next_u64();
		let i = bits as usize & 0xff;

		let u = if symmetric {
			// Convert to a value in the range [2,4) and subtract to get [-1,1)
			// We can't convert to an open range directly, that would require
			// subtracting `3.0 - EPSILON`, which is not representable.
			// It is possible with an extra step, but an open range does not
			// seem necessary for the ziggurat algorithm anyway.
			into_float_with_exponent(bits, 1) - 3.0
		} else {
			// Convert to a value in the range [1,2) and subtract to get (0,1)
			into_float_with_exponent(bits, 0) - (1.0 - f64::EPSILON / 2.0)
		};
		let x = u * x_tab[i];

		let test_x = if symmetric { x.abs() } else { x };

		// algebraically equivalent to |u| < x_tab[i+1]/x_tab[i] (or u < x_tab[i+1]/x_tab[i])
		if test_x < x_tab[i + 1] {
			return x;
		}
		if i == 0 {
			return zero_case(rand, u);
		}
		// algebraically equivalent to f1 + DRanU()*(f0 - f1) < 1
		if f_tab[i + 1] + (f_tab[i] - f_tab[i + 1]) * rand.float01() < pdf(x) {
			return x;
		}
	}
}
