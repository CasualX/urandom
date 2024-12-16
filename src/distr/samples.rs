use super::*;

/// An iterator that generates random values of `T` from the distribution `D`.
///
/// This struct is created by the [`Random::samples`](Random::samples) method. See its documentation for more.
pub struct Samples<'a, R: ?Sized, D, T> {
	rand: &'a mut Random<R>,
	distr: D,
	_phantom: marker::PhantomData<fn() -> T>,
}

impl<'a, R: ?Sized, D, T> Samples<'a, R, D, T> {
	#[inline]
	pub(crate) fn new(rand: &'a mut Random<R>, distr: D) -> Self {
		Samples { rand, distr, _phantom: marker::PhantomData }
	}
}

impl<'a, R: Rng + ?Sized, T, D: Distribution<T>> Iterator for Samples<'a, R, D, T> {
	type Item = T;
	#[inline]
	fn next(&mut self) -> Option<T> {
		Some(self.distr.sample(self.rand))
	}
	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(usize::MAX, None)
	}
}

impl<'a, R: Rng + ?Sized, T, D: Distribution<T>> iter::FusedIterator for Samples<'a, R, D, T> {}
