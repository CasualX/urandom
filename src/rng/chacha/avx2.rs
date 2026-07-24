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
	}}
}

macro_rules! rol {
	($e:expr, $n:literal) => {{
		let e = $e;
		let left = _mm256_slli_epi32(e, $n);
		let right = _mm256_srli_epi32(e, 32 - $n);
		_mm256_or_si256(left, right)
	}};
}
macro_rules! quarter_round {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$a = _mm256_add_epi32($a, $b); $d = rol!(_mm256_xor_si256($d, $a), 16);
		$c = _mm256_add_epi32($c, $d); $b = rol!(_mm256_xor_si256($b, $c), 12);
		$a = _mm256_add_epi32($a, $b); $d = rol!(_mm256_xor_si256($d, $a), 8);
		$c = _mm256_add_epi32($c, $d); $b = rol!(_mm256_xor_si256($b, $c), 7);
	};
}
macro_rules! rotate_matrix {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$b = _mm256_shuffle_epi32($b, (1 << 0) | (2 << 2) | (3 << 4) | (0 << 6));
		$c = _mm256_shuffle_epi32($c, (2 << 0) | (3 << 2) | (0 << 4) | (1 << 6));
		$d = _mm256_shuffle_epi32($d, (3 << 0) | (0 << 2) | (1 << 4) | (2 << 6));
	};
}

#[inline]
pub fn block<const N: usize>(state: &mut super::ChaChaState<N>, ws: &mut [[u32; 16]; 4]) {
	unsafe {
		let words = state.get_state();
		let [xa, xb, xc, xd] = load!(&words);
		let counter1 = _mm_add_epi64(xd, _mm_set_epi64x(0, 1));
		let counter2 = _mm_add_epi64(xd, _mm_set_epi64x(0, 2));
		let counter3 = _mm_add_epi64(xd, _mm_set_epi64x(0, 3));

		let mut a1 = _mm256_broadcastsi128_si256(xa);
		let mut b1 = _mm256_broadcastsi128_si256(xb);
		let mut c1 = _mm256_broadcastsi128_si256(xc);
		let mut d1 = _mm256_setr_m128i(xd, counter1);

		let mut a2 = _mm256_broadcastsi128_si256(xa);
		let mut b2 = _mm256_broadcastsi128_si256(xb);
		let mut c2 = _mm256_broadcastsi128_si256(xc);
		let mut d2 = _mm256_setr_m128i(counter2, counter3);

		let (sa1, sb1, sc1, sd1) = (a1, b1, c1, d1);
		let (sa2, sb2, sc2, sd2) = (a2, b2, c2, d2);

		for _ in 0..N / 2 {
			quarter_round!(a1, b1, c1, d1);
			rotate_matrix!(a1, b1, c1, d1);
			quarter_round!(a1, b1, c1, d1);
			rotate_matrix!(a1, d1, c1, b1);

			quarter_round!(a2, b2, c2, d2);
			rotate_matrix!(a2, b2, c2, d2);
			quarter_round!(a2, b2, c2, d2);
			rotate_matrix!(a2, d2, c2, b2);
		}

		a1 = _mm256_add_epi32(a1, sa1);
		b1 = _mm256_add_epi32(b1, sb1);
		c1 = _mm256_add_epi32(c1, sc1);
		d1 = _mm256_add_epi32(d1, sd1);

		a2 = _mm256_add_epi32(a2, sa2);
		b2 = _mm256_add_epi32(b2, sb2);
		c2 = _mm256_add_epi32(c2, sc2);
		d2 = _mm256_add_epi32(d2, sd2);

		let w11 = _mm256_permute2x128_si256(a1, b1, 0x20); // A1 B1
		let w12 = _mm256_permute2x128_si256(c1, d1, 0x20); // C1 D1
		let w21 = _mm256_permute2x128_si256(a1, b1, 0x31); // A2 B2
		let w22 = _mm256_permute2x128_si256(c1, d1, 0x31); // C2 D2

		let w31 = _mm256_permute2x128_si256(a2, b2, 0x20); // A3 B3
		let w32 = _mm256_permute2x128_si256(c2, d2, 0x20); // C3 D3
		let w41 = _mm256_permute2x128_si256(a2, b2, 0x31); // A4 B4
		let w42 = _mm256_permute2x128_si256(c2, d2, 0x31); // C4 D4

		let ws = ws.as_mut_ptr() as *mut __m256i;
		_mm256_storeu_si256(ws.offset(0), w11);
		_mm256_storeu_si256(ws.offset(1), w12);
		_mm256_storeu_si256(ws.offset(2), w21);
		_mm256_storeu_si256(ws.offset(3), w22);
		_mm256_storeu_si256(ws.offset(4), w31);
		_mm256_storeu_si256(ws.offset(5), w32);
		_mm256_storeu_si256(ws.offset(6), w41);
		_mm256_storeu_si256(ws.offset(7), w42);
	}

	state.set_counter(state.get_counter() + 4);
}
