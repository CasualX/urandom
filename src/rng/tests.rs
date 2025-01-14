use super::*;


#[test]
fn test_trait_object() {
	// Ensure Rng is usable as a trait object
	fn test(rand: &mut Random<dyn Rng>) {
		let _: i32 = rand.next();
	}
	test(&mut crate::new());
	test(&mut crate::seeded(42));
	test(&mut crate::csprng());
}


#[track_caller]
pub fn check_fill_bytes<R: Rng + Clone>(master: &mut Random<R>) {
	master.next_u64();
	master.next_u64();
	master.next_u64();
	master.next_u32();

	let zeroes = [0u8; 256];
	let mut old = [0u8; 256];
	for i in 1..256 {
		let mut rand = master.clone();
		let mut buf = [0u8; 256];
		rand.fill_bytes(&mut buf[..i]);

		// Check the buffer is correctlly filled
		let nzeroes = buf[..i].iter().filter(|&&b| b == 0).count();
		assert!(nzeroes < 4, "too many zeroes in {:?}", &buf[..i]);

		// Check OOB writes
		assert_eq!(buf[i..], zeroes[i..]);

		// Check the output of fill_bytes is consistent
		assert_eq!(buf[..i - 1], old[..i - 1]);
		old = buf;
	}
}

#[cfg(feature = "serde")]
#[track_caller]
pub fn check_serde_initial_state<R: Rng + serde::Serialize + for<'de> serde::Deserialize<'de>>(mut rand: Random<R>) {
	let saved = serde_json::to_string(&rand).unwrap();
	let v1 = rand.next_u64();
	let mut restored: R = serde_json::from_str(&saved).unwrap();
	let v2 = restored.next_u64();
	assert_eq!(v1, v2);
}

#[cfg(feature = "serde")]
#[track_caller]
pub fn check_serde_middle_state<R: Rng + serde::Serialize + for<'de> serde::Deserialize<'de>>(mut rand: Random<R>) {
	let _ = rand.next_u64();
	let saved = serde_json::to_string(&rand).unwrap();
	let v1 = rand.next_u64();
	let mut restored: R = serde_json::from_str(&saved).unwrap();
	let v2 = restored.next_u64();
	assert_eq!(v1, v2);
}
