use super::*;

/// Marks types which can safely be treated as initialized bytes and filled from arbitrary bytes.
///
/// # Safety
///
/// Implementing types must have no uninitialized padding, and accept every possible bit pattern as a valid value.
pub unsafe trait Pod: 'static {}

// These are the only scalar types used as entropy-seeded state in this crate.
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}

unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

/// Views initialized plain data as bytes.
#[inline]
pub fn bytes<T: Pod + ?Sized>(value: &T) -> &[u8] {
	unsafe { slice::from_raw_parts(value as *const T as *const u8, mem::size_of_val(value)) }
}

/// Generates internal plain data from system entropy.
#[cfg(feature = "getrandom")]
#[inline]
pub fn getrandom<T: Pod>() -> T {
	let mut value = MaybeUninit::<T>::uninit();
	getentropy_uninit(slice::from_mut(&mut value));
	unsafe { value.assume_init() }
}

/// Fills initialized internal plain data from system entropy.
#[cfg(feature = "getrandom")]
#[inline]
pub fn getentropy<T: Pod>(buf: &mut [T]) -> &mut [T] {
	let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<MaybeUninit<T>>(), buf.len()) };
	getentropy_uninit(buf)
}

/// Fills uninitialized internal plain data from system entropy.
#[cfg(feature = "getrandom")]
#[inline]
pub fn getentropy_uninit<T: Pod>(buf: &mut [MaybeUninit<T>]) -> &mut [T] {
	let dest = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<MaybeUninit<u8>>(), mem::size_of_val(buf)) };
	super::getentropy_uninit(dest);
	unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<T>(), buf.len()) }
}
