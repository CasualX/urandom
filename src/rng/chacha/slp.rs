// Implementation which tickles LLVM's SLP Vectorizer
// https://llvm.org/docs/Vectorizers.html#the-slp-vectorizer
// Requires `-C opt-level=3 -C target-cpu=native` for best results

use core::mem;

#[inline(always)]
fn u32x4_add(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
	[
		a[0].wrapping_add(b[0]),
		a[1].wrapping_add(b[1]),
		a[2].wrapping_add(b[2]),
		a[3].wrapping_add(b[3]),
	]
}
#[inline(always)]
fn u32x4_xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
	[
		a[0] ^ b[0],
		a[1] ^ b[1],
		a[2] ^ b[2],
		a[3] ^ b[3],
	]
}
#[inline(always)]
fn u32x4_rol(a: [u32; 4], n: u32) -> [u32; 4] {
	// FIXME! SLP vectorizer can't handle u32 right shifts
	[
		a[0] << n | a[0] >> (32 - n),
		a[1] << n | a[1] >> (32 - n),
		a[2] << n | a[2] >> (32 - n),
		a[3] << n | a[3] >> (32 - n),
	]
}

macro_rules! quarter_round {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$a = u32x4_add($a, $b); $d = u32x4_rol(u32x4_xor($d, $a), 16);
		$c = u32x4_add($c, $d); $b = u32x4_rol(u32x4_xor($b, $c), 12);
		$a = u32x4_add($a, $b); $d = u32x4_rol(u32x4_xor($d, $a), 8);
		$c = u32x4_add($c, $d); $b = u32x4_rol(u32x4_xor($b, $c), 7);
	};
}
macro_rules! rotate_matrix {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$a = [$a[0], $a[1], $a[2], $a[3]];
		$b = [$b[1], $b[2], $b[3], $b[0]];
		$c = [$c[2], $c[3], $c[0], $c[1]];
		$d = [$d[3], $d[0], $d[1], $d[2]];
	};
}

#[inline]
pub fn block<const N: usize>(state: &mut super::ChaChaState<N>, ws: &mut [[u32; 16]; 4]) {
	{
		let words1 = state.get_state();
		let [mut a1, mut b1, mut c1, mut d1] = words1;
		let words2 = state.add_counter(1).get_state();
		let [mut a2, mut b2, mut c2, mut d2] = words2;
		let words3 = state.add_counter(2).get_state();
		let [mut a3, mut b3, mut c3, mut d3] = words3;
		let words4 = state.add_counter(3).get_state();
		let [mut a4, mut b4, mut c4, mut d4] = words4;

		for _ in 0..N / 2 {
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

		let ws: &mut [[[u32; 4]; 4]; 4] = unsafe { mem::transmute(ws) };

		a1 = u32x4_add(a1, words1[0]);
		b1 = u32x4_add(b1, words1[1]);
		c1 = u32x4_add(c1, words1[2]);
		d1 = u32x4_add(d1, words1[3]);
		ws[0] = [a1, b1, c1, d1];

		a2 = u32x4_add(a2, words2[0]);
		b2 = u32x4_add(b2, words2[1]);
		c2 = u32x4_add(c2, words2[2]);
		d2 = u32x4_add(d2, words2[3]);
		ws[1] = [a2, b2, c2, d2];

		a3 = u32x4_add(a3, words3[0]);
		b3 = u32x4_add(b3, words3[1]);
		c3 = u32x4_add(c3, words3[2]);
		d3 = u32x4_add(d3, words3[3]);
		ws[2] = [a3, b3, c3, d3];

		a4 = u32x4_add(a4, words4[0]);
		b4 = u32x4_add(b4, words4[1]);
		c4 = u32x4_add(c4, words4[2]);
		d4 = u32x4_add(d4, words4[3]);
		ws[3] = [a4, b4, c4, d4];
	}

	state.set_counter(state.get_counter() + 4);
}
