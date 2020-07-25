#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

macro_rules! load {
	($words:expr) => {{
		let words = $words as *const _ as *const __m128i;
		let a = _mm_loadu_si128(words.offset(0));
		let b = _mm_loadu_si128(words.offset(1));
		let c = _mm_loadu_si128(words.offset(2));
		let d = _mm_loadu_si128(words.offset(3));
		[a, b, c, d]
	}};
}
macro_rules! store {
	($words:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
		let words = $words as *mut _ as *mut __m128i;
		_mm_storeu_si128(words.offset(0), $a);
		_mm_storeu_si128(words.offset(1), $b);
		_mm_storeu_si128(words.offset(2), $c);
		_mm_storeu_si128(words.offset(3), $d);
	};
}
macro_rules! rol {
	($e:expr, $n:literal) => {{
		let e = $e;
		let left = _mm_slli_epi32(e, $n);
		let right = _mm_srli_epi32(e, 32 - $n);
		_mm_or_si128(left, right)
	}};
}
macro_rules! quarter_round {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$a = _mm_add_epi32($a, $b); $d = rol!(_mm_xor_si128($d, $a), 16);
		$c = _mm_add_epi32($c, $d); $b = rol!(_mm_xor_si128($b, $c), 12);
		$a = _mm_add_epi32($a, $b); $d = rol!(_mm_xor_si128($d, $a), 8);
		$c = _mm_add_epi32($c, $d); $b = rol!(_mm_xor_si128($b, $c), 7);
	};
}
macro_rules! rotate_matrix {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$b = _mm_shuffle_epi32($b, (1 << 0) | (2 << 2) | (3 << 4) | (0 << 6));
		$c = _mm_shuffle_epi32($c, (2 << 0) | (3 << 2) | (0 << 4) | (1 << 6));
		$d = _mm_shuffle_epi32($d, (3 << 0) | (0 << 2) | (1 << 4) | (2 << 6));
	};
}

#[inline(never)]
pub fn block(state: &mut [u32; 16], ws: &mut [u32; 16]) {
	unsafe {
		let [mut a, mut b, mut c, mut d] = load!(state);

		for _ in 0..10 {
			// column rounds
			quarter_round!(a, b, c, d);
			// diagonal rounds
			rotate_matrix!(a, b, c, d);
			quarter_round!(a, b, c, d);
			rotate_matrix!(a, d, c, b);
		}

		// add unscrambled block to prevent invertibility
		let [sa, sb, sc, sd] = load!(state);
		a = _mm_add_epi32(a, sa);
		b = _mm_add_epi32(b, sb);
		c = _mm_add_epi32(c, sc);
		d = _mm_add_epi32(d, sd);

		store!(ws, a, b, c, d);
	}

	super::increment_counter(state);
}
