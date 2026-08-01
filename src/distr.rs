/*!
Generating random samples from probability distributions.

This module is the home of the [`Distribution`] trait and several of its implementations.
It is the workhorse behind some of the convenient functionality of the [`Random`] struct,
e.g. [`Random::random`], [`Random::uniform`] and of course [`Random::sample`].

Abstractly, a [probability distribution] describes the probability of occurrence of each value in its sample space.

More concretely, an implementation of `Distribution<T>` for type `D` is an algorithm for choosing values from the sample space (a subset of `T`)
according to the distribution `D` represents, using mutable randomness supplied through [`Random`].

# The `StandardUniform` distribution

The [`StandardUniform`] distribution is important to mention.
This is the distribution used by [`Random::random`] and represents the "default" way to produce a random value for many different types,
including most primitive types, tuples, arrays, and a few derived types. See the documentation of [`StandardUniform`] for more details.

Implementing `Distribution<T>` for [`StandardUniform`] for user types `T` makes it possible to generate type `T` with [`Random::random`].

# The `Uniform` distribution

The [`Uniform`] distribution is similar to the [`StandardUniform`] distribution
but it allows the sample space to be specified as an arbitrary range within its target type `T`.
Both [`StandardUniform`] and [`Uniform`] are in some sense uniform distributions.

Values may be sampled from this distribution using [`Random::uniform`] or by creating a distribution object from a `low..high` or `low..=high`.
When the range limits are not known at compile time it is typically faster to reuse an existing distribution object than to call [`Random::uniform`].

User-defined types can support [`Uniform`] sampling by implementing [`SampleUniform`] and providing a corresponding [`UniformSampler`].
This enables values of the type to be generated with [`Random::uniform`]. See the [`Uniform`] documentation for a complete example.

# Reproducibility policy

Distribution implementations provided by this crate preserve their observable behavior between patch releases of the
same minor version. Algorithms are not changed in ways that alter their output, including solely for performance.
They may change in a new minor release.

This is a best-effort policy because results can also depend on the execution platform. Sampling based only on
integer operations is generally predictable across supported targets. Floating-point results are more sensitive
to platform behavior, particularly for math-heavy distributions. Uniform floating-point sampling is predictable
on targets with Rust's strict IEEE 754 floating-point semantics.

For reproducible recordings, simulations, or game replays, pin urandom to one minor release (for example, `~1.0`)
and keep relevant external inputs and platform behavior consistent. Individual distributions may document a
stronger guarantee or additional limitations.

[probability distribution]: https://en.wikipedia.org/wiki/Probability_distribution
*/

use core::{fmt, iter, marker, ops};
use crate::{Random, Rng};

mod alnum;
mod bernoulli;
mod dice;
mod float01;
mod samples;
mod standard;
mod uniform;

pub use self::alnum::Alnum;
pub use self::bernoulli::Bernoulli;
pub use self::dice::Dice;
pub use self::float01::Float01;
pub use self::samples::Samples;
pub use self::standard::StandardUniform;
pub use self::uniform::*;

cfg_select! {
	feature = "std" => {
		mod exp;
		mod normal;
		mod triangular;
		mod ziggurat_tables;
		mod ziggurat;

		pub use self::exp::{Exp, Exp1, ExpError};
		pub use self::normal::{LogNormal, Normal, NormalError, StandardNormal};
		pub use self::triangular::{Triangular, TriangularError};
	}
	_ => {}
}

/// Types (distributions) that can be used to create a random instance of `T`.
///
/// It is possible to sample from a distribution through both the
/// `Distribution` trait and [`Random`] struct, via `distr.sample(&mut rand)` and
/// `rand.sample(&distr)`. There's also the [`Random::samples`] method, which
/// produces an iterator that samples from the distribution.
///
/// Sampling only requires a shared reference to the distribution, while mutable randomness is supplied by [`Random`].
/// This makes a configured distribution convenient to reuse without requiring mutable access to it,
/// and for most distributions efficient stateless sampling algorithms are available.
///
/// Implementations provided by this crate follow the module-level [reproducibility policy](crate::distr#reproducibility-policy).
/// This policy does not apply to downstream implementations of this trait.
pub trait Distribution<T> {
	/// Generate a random value of `T`, using rand as the source of randomness.
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> T;

	/// Creates a distribution of values of `U` by mapping the output of `Self` through the closure `F`
	///
	/// # Examples
	///
	/// ```
	/// use urandom::distr::{Dice, Distribution};
	///
	#[cfg_attr(feature = "getrandom", doc = "let mut rand = urandom::new();")]
	#[cfg_attr(not(feature = "getrandom"), doc = "let mut rand = urandom::seeded(42);")]
	///
	/// let even_number = Dice::D6.map(|num| num % 2 == 0);
	/// while !rand.sample(&even_number) {
	/// 	println!("Still odd; rolling again!");
	/// }
	/// ```
	#[inline]
	fn map<U, F: Fn(T) -> U>(self, f: F) -> Map<Self, F, T, U> where Self: Sized {
		Map { distr: self, f, _phantom: marker::PhantomData }
	}
}

impl<'a, T, D: Distribution<T> + ?Sized> Distribution<T> for &'a D {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> T {
		(*self).sample(rand)
	}
}

/// Distribution of values of type `U` derived from the distribution `D`.
///
/// This struct is created by the [`Distribution::map`] method.
/// See its documentation for more.
pub struct Map<D, F, T, U> {
	distr: D,
	f: F,
	_phantom: marker::PhantomData<fn(T) -> U>,
}

impl<T, D: Distribution<T>, U, F: Fn(T) -> U> Distribution<U> for Map<D, F, T, U> {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> U {
		(self.f)(self.distr.sample(rand))
	}
}
