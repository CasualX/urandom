use core::arch::aarch64::*;

macro_rules! load {
	($words:expr) => {{
		let words = $words as *const _ as *const u32;
		let a = vld1q_u32(words.add(0));
		let b = vld1q_u32(words.add(4));
		let c = vld1q_u32(words.add(8));
		let d = vld1q_u32(words.add(12));
		[a, b, c, d]
	}};
}

macro_rules! rol {
	($value:expr, 16) => {{
		let value = $value;
		vreinterpretq_u32_u16(vrev32q_u16(vreinterpretq_u16_u32(value)))
	}};
	($value:expr, $amount:literal) => {{
		let value = $value;
		vsliq_n_u32(vshrq_n_u32(value, 32 - $amount), value, $amount)
	}};
}

macro_rules! quarter_round {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$a = vaddq_u32($a, $b); $d = rol!(veorq_u32($d, $a), 16);
		$c = vaddq_u32($c, $d); $b = rol!(veorq_u32($b, $c), 12);
		$a = vaddq_u32($a, $b); $d = rol!(veorq_u32($d, $a), 8);
		$c = vaddq_u32($c, $d); $b = rol!(veorq_u32($b, $c), 7);
	};
}

macro_rules! rotate_matrix {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$b = vextq_u32($b, $b, 1);
		$c = vextq_u32($c, $c, 2);
		$d = vextq_u32($d, $d, 3);
	};
}

macro_rules! finalize {
	($dest:expr, $a:expr, $b:expr, $c:expr, $d:expr, $words:expr) => {
		let [sa, sb, sc, sd] = load!($words);
		$a = vaddq_u32($a, sa);
		$b = vaddq_u32($b, sb);
		$c = vaddq_u32($c, sc);
		$d = vaddq_u32($d, sd);
		let dest = $dest as *mut _ as *mut u32;
		vst1q_u32(dest.add(0), $a);
		vst1q_u32(dest.add(4), $b);
		vst1q_u32(dest.add(8), $c);
		vst1q_u32(dest.add(12), $d);
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
