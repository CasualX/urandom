use super::*;

#[inline(always)]
pub fn rng_f32(seed: u32) -> f32 {
	f32::from_bits(127 << (f32::MANTISSA_DIGITS - 1) | (seed >> 9))
}

#[inline(always)]
pub fn rng_f64(seed: u64) -> f64 {
	f64::from_bits(1023 << (f64::MANTISSA_DIGITS - 1) | (seed >> 12))
}

#[inline(always)]
pub fn rng_fill_bytes<R: Rng>(mut rng: R, buf: &mut [MaybeUninit<u8>]) -> R {
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

#[inline]
pub fn getrandom<T: dataview::Pod>() -> T {
	let mut value = MaybeUninit::<T>::uninit();
	getentropy_uninit(slice::from_mut(&mut value));
	unsafe { value.assume_init() }
}

#[inline]
pub fn fill_bytes<'a, R: Rng + ?Sized, T: dataview::Pod>(rng: &mut R, buf: &'a mut [T]) -> &'a mut [T] {
	unsafe {
		let buf_bytes = slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut MaybeUninit<u8>, mem::size_of_val(buf));
		rng.fill_bytes(buf_bytes);
		buf
	}
}

#[inline]
pub fn fill_bytes_uninit<'a, R: Rng + ?Sized, T: dataview::Pod>(rng: &mut R, buf: &'a mut [core::mem::MaybeUninit<T>]) -> &'a mut [T] {
	unsafe {
		let buf_bytes = slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut MaybeUninit<u8>, mem::size_of_val(buf));
		rng.fill_bytes(buf_bytes);
		mem::transmute(buf)
	}
}

#[inline]
pub fn random_bytes<R: Rng + ?Sized, T: dataview::Pod>(rng: &mut R) -> T {
	let mut value = MaybeUninit::<T>::uninit();
	fill_bytes_uninit(rng, slice::from_mut(&mut value));
	unsafe { value.assume_init() }
}
