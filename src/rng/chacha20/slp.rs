// Implementation which tickles LLVM's SLP Vectorizer
// https://llvm.org/docs/Vectorizers.html#the-slp-vectorizer
// Requires `-C opt-level=3 -C target-cpu=native` for best results

use dataview::Pod;

#[inline(always)]
fn u32x4_add(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
	[a[0].wrapping_add(b[0]), a[1].wrapping_add(b[1]), a[2].wrapping_add(b[2]), a[3].wrapping_add(b[3])]
}
#[inline(always)]
fn u32x4_xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
	[a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}
#[inline(always)]
fn u32x4_rol(a: [u32; 4], n: u32) -> [u32; 4] {
	[a[0].rotate_left(n), a[1].rotate_left(n), a[2].rotate_left(n), a[3].rotate_left(n) ]
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

#[inline(never)]
pub fn block(state: &mut [u32; 16], ws: &mut [u32; 16]) {
	{
		let state: &mut [[u32; 4]; 4] = state.as_data_view_mut().read_mut(0);
		let [mut a, mut b, mut c, mut d] = state;

		for _ in 0..10 {
			// column rounds
			quarter_round!(a, b, c, d);
			// diagonal rounds
			rotate_matrix!(a, b, c, d);
			quarter_round!(a, b, c, d);
			rotate_matrix!(a, d, c, b);
		}

		a = u32x4_add(a, state[0]);
		b = u32x4_add(b, state[1]);
		c = u32x4_add(c, state[2]);
		d = u32x4_add(d, state[3]);

		let ws: &mut [[u32; 4]; 4] = ws.as_data_view_mut().read_mut(0);
		*ws = [a, b, c, d];
	}

	super::increment_counter(state);
}
