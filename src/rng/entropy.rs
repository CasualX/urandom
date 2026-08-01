use super::*;

/// Fills a byte buffer with cryptographically secure entropy.
///
/// # Panics
///
/// If unable to provide secure entropy this method will panic.
///
/// # Implementation notes
///
/// The implementation is provided by the [`getrandom`](https://crates.io/crates/getrandom) crate.
#[inline]
pub fn getentropy(buf: &mut [u8]) -> &mut [u8] {
	let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<MaybeUninit<u8>>(), buf.len()) };
	getentropy_uninit(buf)
}

/// Fills an uninitialized byte buffer with cryptographically secure entropy.
///
/// # Panics
///
/// If unable to provide secure entropy this method will panic.
///
/// # Implementation notes
///
/// The implementation is provided by the [`getrandom`](https://crates.io/crates/getrandom) crate.
#[inline]
pub fn getentropy_uninit(buf: &mut [MaybeUninit<u8>]) -> &mut [u8] {
	match getrandom::fill_uninit(buf) {
		Ok(buf) => buf,
		Err(_) => getentropy_not_ready(),
	}
}

#[cold]
fn getentropy_not_ready() -> ! {
	panic!("getentropy not ready")
}
