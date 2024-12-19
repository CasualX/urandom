#![allow(dead_code)]
use super::*;

#[inline(always)]
pub fn rng_f32(seed: u32) -> f32 {
	f32::from_bits(0b0_01111111 << (f32::MANTISSA_DIGITS - 1) | (seed >> 9))
}

#[inline(always)]
pub fn rng_f64(seed: u64) -> f64 {
	f64::from_bits(0b0_01111111111 << (f64::MANTISSA_DIGITS - 1) | (seed >> 12))
}

#[inline(always)]
pub fn fill_bytes<R: Rng>(mut rng: R, buf: &mut [MaybeUninit<u8>]) -> R {
	unsafe {
		let mut ptr = buf.as_mut_ptr() as *mut u8;
		let mut len = buf.len();

		// Loop unrolled for eight bytes at the time
		while len >= 8 {
			let value = rng.next_u64();
			// Unaligned u64 little-endian write
			ptr::copy_nonoverlapping(value.to_le_bytes().as_ptr(), ptr, 8);
			ptr = ptr.add(8);
			len -= 8;
		}

		if len > 0 {
			let mut value = rng.next_u64();

			if len >= 4 {
				// Unaligned u32 little-endian write
				ptr::copy_nonoverlapping((value as u32).to_le_bytes().as_ptr(), ptr, 4);
				ptr = ptr.add(4);
				len -= 4;
				value >>= 32;
			}

			if len >= 2 {
				// Unaligned u16 little-endian write
				ptr.add(0).write(((value >> 0) & 0xff) as u8);
				ptr.add(1).write(((value >> 8) & 0xff) as u8);
				ptr = ptr.add(2);
				len -= 2;
				value >>= 16;
			}

			if len >= 1 {
				ptr.add(0).write((value & 0xff) as u8);
			}
		}
	}
	rng
}

/// Polyfill for `maybe_uninit_slice` feature's `MaybeUninit::slice_assume_init_mut`.
/// Every element of `slice` must have been initialized.
#[inline(always)]
pub unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
	// SAFETY: `MaybeUninit<T>` is guaranteed to be layout-compatible with `T`.
	&mut *(slice as *mut [MaybeUninit<T>] as *mut [T])
}

#[inline(always)]
pub fn slice_as_uninit<T>(slice: &[T]) -> &[MaybeUninit<T>] {
	// SAFETY: `MaybeUninit<T>` is guaranteed to be layout-compatible with `T`.
	// There is no risk of writing a `MaybeUninit<T>` into the result since the result isn't mutable.
	unsafe { &*(slice as *const [T] as *const [MaybeUninit<T>]) }
}

/// View an mutable initialized array as potentially-uninitialized.
///
/// This is unsafe because it allows assigning uninitialized values into `slice`, which would be undefined behavior.
#[inline(always)]
pub unsafe fn slice_as_uninit_mut<T>(slice: &mut [T]) -> &mut [MaybeUninit<T>] {
	// SAFETY: `MaybeUninit<T>` is guaranteed to be layout-compatible with `T`.
	&mut *(slice as *mut [T] as *mut [MaybeUninit<T>])
}

#[inline]
pub fn getrandom<T: dataview::Pod>() -> T {
	unsafe {
		let mut value = MaybeUninit::<T>::uninit();
		getentropy_uninit(slice::from_raw_parts_mut(&mut value as *mut _ as *mut MaybeUninit<u8>, mem::size_of::<T>()));
		value.assume_init()
	}
}
