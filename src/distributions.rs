/*!
Generating random samples from probability distributions.

This module is the home of the [`Distribution`] trait and several of its implementations.
It is the workhorse behind some of the convenient functionality of the [`Random`] struct,
e.g. [`Random::next`], [`Random::range`] and of course [`Random::sample`].

Abstractly, a [probability distribution] describes the probability of occurance of each value in its sample space.

More concretely, an implementation of `Distribution<T>` for type `X` is an algorithm for choosing values from the sample space (a subset of `T`)
according to the distribution `X` represents, using an external source of randomness (an Rng supplied to the `sample` function).

A type `X` may implement `Distribution<T>` for multiple types `T`.
Any type implementing [`Distribution`] is stateless (i.e. immutable), but it may have internal parameters set at construction time
(for example, [`Uniform`] allows specification of its sample space as a range within `T`).

# The `Standard` distribution

The [`Standard`] distribution is important to mention.
This is the distribution used by [`Random::next`] and represents the "default" way to produce a random value for many different types,
including most primitive types, tuples, arrays, and a few derived types. See the documentation of [`Standard`] for more details.

Implementing `Distribution<T>` for [`Standard`] for user types `T` makes it possible to generate type `T` with [`Random::next`].

# The `Uniform` distribution

The [`Uniform`] distribution is similar to the [`Standard`] distribution
but it allows the sample space to be specified as an arbitrary range within its target type `T`.
Both [`Standard`] and [`Uniform`] are in some sense uniform distributions.

Values may be sampled from this distribution using [`Random::range`] or by creating a distribution object `From<Range>` or `From<RangeInclusive>`.
When the range limits are not known at compile time it is typically faster to reuse an existing distribution object than to call [`Random::range`].

User types `T` may also implement `Distribution<T>` for [`Uniform`], although this is less straightforward than for [`Standard`]
(see the documentation in the uniform module. Doing so enables generation of values of type `T` with [`Random::range`].

[`Distribution`]: trait.Distribution.html
[`Random`]: ../struct.Random.html
[`Random::next`]: ../struct.Random.html#method.next
[`Random::range`]: ../struct.Random.html#method.range
[`Random::sample`]: ../struct.Random.html#method.sample
[probability distribution]: https://en.wikipedia.org/wiki/Probability_distribution
[`Uniform`]: struct.Uniform.html
[`Standard`]: struct.Standard.html
*/

use crate::{Random, Rng};

mod standard;
mod uniform;
mod float01;
mod bernoulli;
mod choose;
mod dice;
mod alphanumeric;

pub use self::standard::Standard;
pub use self::uniform::*;
pub use self::float01::Float01;
pub use self::bernoulli::Bernoulli;
pub use self::choose::Choose;
pub use self::dice::Dice;
pub use self::alphanumeric::Alphanumeric;

/// Types (distributions) that can be used to create a random instance of `T`.
///
/// It is possible to sample from a distribution through both the
/// `Distribution` trait and [`Random`] struct, via `distr.sample(&mut rng)` and
/// `rng.sample(&distr)`. There's also the [`Random::samples`] method, which
/// produces an iterator that samples from the distribution.
///
/// All implementations are expected to be immutable; this has the significant advantage of not needing to consider thread safety,
/// and for most distributions efficient state-less sampling algorithms are available.
///
/// Implementations are typically expected to be portable with reproducible results when used with a PRNG with fixed seed;
/// see the [portability chapter] of The Rust Rand Book. In some cases this does not apply,
/// e.g. the `usize` type requires different sampling on 32-bit and 64-bit machines.
///
/// [`Random`]: ../struct.Random.html
/// [`Random::samples`]: ../struct.Random.html#method.samples
/// [portability chapter]: https://rust-random.github.io/book/portability.html
pub trait Distribution<T> {
	/// Generate a random value of `T`, using rng as the source of randomness.
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> T;
}

mod samples;
pub use self::samples::Samples;
