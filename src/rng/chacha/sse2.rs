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
macro_rules! finalize {
	($dest:expr, $a:expr, $b:expr, $c:expr, $d:expr, $words:expr) => {
		let [sa, sb, sc, sd] = load!($words);
		$a = _mm_add_epi32($a, sa);
		$b = _mm_add_epi32($b, sb);
		$c = _mm_add_epi32($c, sc);
		$d = _mm_add_epi32($d, sd);
		let dest = $dest as *mut _ as *mut __m128i;
		_mm_storeu_si128(dest.offset(0), $a);
		_mm_storeu_si128(dest.offset(1), $b);
		_mm_storeu_si128(dest.offset(2), $c);
		_mm_storeu_si128(dest.offset(3), $d);
	};
}

// #[target_feature(enable = "sse2")]
#[inline]
pub fn block(state: &mut super::ChaChaCore, ws: &mut [[u32; 16]; 4], n: usize) {
	unsafe {
		let words1 = state.get_state();
		let [mut a1, mut b1, mut c1, mut d1] = load!(&words1);

		let words2 = state.add_counter(1).get_state();
		let [mut a2, mut b2, mut c2, mut d2] = load!(&words2);

		let words3 = state.add_counter(2).get_state();
		let [mut a3, mut b3, mut c3, mut d3] = load!(&words3);

		let words4 = state.add_counter(3).get_state();
		let [mut a4, mut b4, mut c4, mut d4] = load!(&words4);

		for _ in 0..n / 2 {
			quarter_round!(a1, b1, c1, d1);
			rotate_matrix!(a1, b1, c1, d1);
			quarter_round!(a1, b1, c1, d1);
			rotate_matrix!(a1, d1, c1, b1);

			quarter_round!(a2, b2, c2, d2);
			rotate_matrix!(a2, b2, c2, d2);
			quarter_round!(a2, b2, c2, d2);
			rotate_matrix!(a2, d2, c2, b2);

			quarter_round!(a3, b3, c3, d3);
			rotate_matrix!(a3, b3, c3, d3);
			quarter_round!(a3, b3, c3, d3);
			rotate_matrix!(a3, d3, c3, b3);

			quarter_round!(a4, b4, c4, d4);
			rotate_matrix!(a4, b4, c4, d4);
			quarter_round!(a4, b4, c4, d4);
			rotate_matrix!(a4, d4, c4, b4);
		}

		finalize!(&mut ws[0], a1, b1, c1, d1, &words1);
		finalize!(&mut ws[1], a2, b2, c2, d2, &words2);
		finalize!(&mut ws[2], a3, b3, c3, d3, &words3);
		finalize!(&mut ws[3], a4, b4, c4, d4, &words4);
	}

	state.set_counter(state.get_counter() + 4);
}
