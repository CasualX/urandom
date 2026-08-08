use core::arch::wasm32::*;

macro_rules! load {
	($words:expr) => {{
		let words = $words as *const _ as *const v128;
		let a = v128_load(words.add(0));
		let b = v128_load(words.add(1));
		let c = v128_load(words.add(2));
		let d = v128_load(words.add(3));
		[a, b, c, d]
	}};
}

macro_rules! rol {
	($value:expr, $amount:literal) => {{
		let value = $value;
		v128_or(i32x4_shl(value, $amount), u32x4_shr(value, 32 - $amount))
	}};
}

macro_rules! quarter_round {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$a = i32x4_add($a, $b); $d = rol!(v128_xor($d, $a), 16);
		$c = i32x4_add($c, $d); $b = rol!(v128_xor($b, $c), 12);
		$a = i32x4_add($a, $b); $d = rol!(v128_xor($d, $a), 8);
		$c = i32x4_add($c, $d); $b = rol!(v128_xor($b, $c), 7);
	};
}

macro_rules! rotate_matrix {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$b = i32x4_shuffle::<1, 2, 3, 0>($b, $b);
		$c = i32x4_shuffle::<2, 3, 0, 1>($c, $c);
		$d = i32x4_shuffle::<3, 0, 1, 2>($d, $d);
	};
}

macro_rules! finalize {
	($dest:expr, $a:expr, $b:expr, $c:expr, $d:expr, $words:expr) => {
		let [sa, sb, sc, sd] = load!($words);
		$a = i32x4_add($a, sa);
		$b = i32x4_add($b, sb);
		$c = i32x4_add($c, sc);
		$d = i32x4_add($d, sd);
		let dest = $dest as *mut _ as *mut v128;
		v128_store(dest.add(0), $a);
		v128_store(dest.add(1), $b);
		v128_store(dest.add(2), $c);
		v128_store(dest.add(3), $d);
	};
}

#[inline]
pub fn block<const N: usize>(state: &mut super::ChaChaState<N>, output: &mut [[u32; 16]; 4]) {
	unsafe {
		let words1 = state.get_state();
		let [mut a1, mut b1, mut c1, mut d1] = load!(&words1);

		let words2 = state.add_counter(1).get_state();
		let [mut a2, mut b2, mut c2, mut d2] = load!(&words2);

		let words3 = state.add_counter(2).get_state();
		let [mut a3, mut b3, mut c3, mut d3] = load!(&words3);

		let words4 = state.add_counter(3).get_state();
		let [mut a4, mut b4, mut c4, mut d4] = load!(&words4);

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

		finalize!(&mut output[0], a1, b1, c1, d1, &words1);
		finalize!(&mut output[1], a2, b2, c2, d2, &words2);
		finalize!(&mut output[2], a3, b3, c3, d3, &words3);
		finalize!(&mut output[3], a4, b4, c4, d4, &words4);
	}

	state.set_counter(state.get_counter().wrapping_add(4));
}
