use core::iter;
use core::marker::PhantomData;
use crate::{Distribution, Random, Rng};

/// An iterator that generates random values of `T` with distribution `D`, using `R` as the source of randomness.
///
/// This struct is created by the [`Random::samples`](Random::samples) method. See its documentation for more.
pub struct Samples<'a, R: ?Sized, D, T> {
	rng: &'a mut Random<R>,
	distr: D,
	_phantom: PhantomData<fn() -> T>,
}
impl<'a, R: ?Sized, D, T> Samples<'a, R, D, T> {
	#[inline]
	pub(crate) fn new(rng: &'a mut Random<R>, distr: D) -> Self {
		Samples { rng, distr, _phantom: PhantomData }
	}
}
impl<'a, R: ?Sized, D, T> Iterator for Samples<'a, R, D, T> where R: Rng, D: Distribution<T> {
	type Item = T;
	#[inline]
	fn next(&mut self) -> Option<T> {
		Some(self.distr.sample(self.rng))
	}
	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(usize::MAX, None)
	}
}
impl<'a, R: ?Sized, D, T> iter::FusedIterator for Samples<'a, R, D, T> where R: Rng, D: Distribution<T> {}
