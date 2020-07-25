
macro_rules! quarter_round {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		$a = $a.wrapping_add($b); $d = ($d ^ $a).rotate_left(16);
		$c = $c.wrapping_add($d); $b = ($b ^ $c).rotate_left(12);
		$a = $a.wrapping_add($b); $d = ($d ^ $a).rotate_left(8);
		$c = $c.wrapping_add($d); $b = ($b ^ $c).rotate_left(7);
	};
}

#[inline(never)]
pub fn block(state: &mut [u32; 16], ws: &mut [u32; 16]) {
	*ws = *state;

	for _ in 0..10 {
		// column rounds
		quarter_round!(ws[0], ws[4], ws[8], ws[12]);
		quarter_round!(ws[1], ws[5], ws[9], ws[13]);
		quarter_round!(ws[2], ws[6],ws[10], ws[14]);
		quarter_round!(ws[3], ws[7],ws[11], ws[15]);
		// diagonal rounds
		quarter_round!(ws[0], ws[5],ws[10], ws[15]);
		quarter_round!(ws[1], ws[6],ws[11], ws[12]);
		quarter_round!(ws[2], ws[7], ws[8], ws[13]);
		quarter_round!(ws[3], ws[4], ws[9], ws[14]);
	}

	for i in 0..16 {
		ws[i] = ws[i].wrapping_add(state[i]);
	}

	super::increment_counter(state);
}
