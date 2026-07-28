use super::*;
use core::time::Duration;

#[test]
fn test_usize_matches_u64() {
	let highs = [1usize, 2, 3, 255, 65_537, u32::MAX as usize];
	for high in highs {
		let mut actual = crate::seeded(high as u64);
		let mut model = crate::seeded(high as u64);
		for _ in 0..100 {
			let value = actual.uniform(0usize..high);
			let expected = model.uniform(0u64..high as u64) as usize;
			assert_eq!(value, expected, "high {high}");
		}
	}
}

#[test]
fn test_float_constructors() {
	assert!(matches!(Uniform::try_new(0.0f32, f32::INFINITY), Err(UniformError::NonFinite)));
	assert!(matches!(Uniform::try_new_inclusive(f64::NAN, 1.0), Err(UniformError::NonFinite)));

	let _ = Uniform::new(0.0f32, f32::INFINITY);
	let _ = Uniform::new_inclusive(f64::NAN, 1.0);
	let _: Uniform<f32> = (0.0..f32::INFINITY).into();
	let _: Uniform<f64> = (f64::NAN..=1.0).into();

	let _ = <Uniform<f32> as UniformSampler<f32>>::new(0.0, f32::INFINITY);
	let _ = <Uniform<f64> as UniformSampler<f64>>::new_inclusive(f64::NAN, 1.0);
}

#[test]
fn test_bias() {
	let distr = Uniform::new_inclusive(0u32, 0xC0000000);
	println!("distr: {distr:#x?}");

	let mut rand = crate::new();
	let mut buckets = [0u32; 3];

	for value in rand.samples(distr).take(100000) {
		if value < 0x40000000 {
			buckets[0] += 1;
		}
		else if value < 0x80000000 {
			buckets[1] += 1;
		}
		else if value <= 0xC0000000 {
			buckets[2] += 1;
		}
		else {
			panic!("value: {:#x}", value);
		}
	}

	let mean = (buckets[0] as i64 + buckets[1] as i64 + buckets[2] as i64) / 3;
	let pass = buckets.iter().all(|&odd| (odd as i64 - mean).abs() < 1000);
	println!("mean:{mean} buckets:{buckets:?} pass:{pass}");
	assert!(pass);
}

#[test]
fn test_edges_large() {
	let distr = Uniform::new_inclusive(u32::MIN, u32::MAX);
	println!("distr: {distr:#x?}");
	let mut rand = crate::new();
	let mut zeros = 0;
	for value in rand.samples(distr).take(100000) {
		if value == 0 {
			zeros += 1;
		}
	}
	assert!(zeros < 5, "found {zeros} zero samples!");
}

#[test]
fn test_edges_small() {
	let distr1 = Uniform::new_inclusive(10, 10);
	let distr2 = Uniform::new(23, 24);
	let mut rand = crate::new();
	for value1 in rand.samples(distr1).take(100) {
		assert_eq!(value1, 10);
	}
	for value2 in rand.samples(distr2).take(100) {
		assert_eq!(value2, 23);
	}
}

#[test]
fn test_yolo() {
	let mut rand = crate::new();
	for _ in 0..10000 {
		let mut low: i16 = rand.random();
		let mut high: i16 = rand.random();
		if high < low {
			let tmp = low;
			low = high;
			high = tmp;
		}
		let value = rand.uniform(low..=high);
		assert!(value >= low && value <= high);
		if low != high {
			let value = rand.uniform(low..high);
			assert!(value >= low && value < high);
		}
	}
}

#[test]
fn test_char() {
	let mut rand = crate::seeded(42);
	for _ in 0..10000 {
		let value = rand.uniform('\u{D7F0}'..='\u{E010}');
		assert!(('\u{D7F0}'..='\u{D7FF}').contains(&value) || ('\u{E000}'..='\u{E010}').contains(&value));
	}

	for _ in 0..10000 {
		let _: char = rand.uniform('\0'..=char::MAX);
	}
}

#[test]
fn test_duration() {
	let low = Duration::new(10, 50000);
	let high = Duration::new(100, 1234);
	let mut rand = crate::seeded(42);
	for _ in 0..10000 {
		let value = rand.uniform(low..high);
		assert!(value >= low && value < high);
	}

	let exact = Duration::new(u64::MAX, 999_999_999);
	for _ in 0..10 {
		assert_eq!(rand.uniform(exact..=exact), exact);
	}
}

#[test]
fn maximum_duration_singleton_does_not_overflow() {
	let max = Duration::new(u64::MAX, 999_999_999);
	let distr = UniformDuration::try_new_inclusive(max, max).unwrap();
	let mut rand = crate::rng::MockRng::slice(&[u64::MAX, u64::MAX]);
	assert_eq!(distr.sample(&mut rand), max);
}

#[cfg(feature = "serde")]
#[test]
fn test_char_serde() {
	let chars = Uniform::new('\u{D7F0}', '\u{E010}');
	let saved = serde_json::to_string(&chars).unwrap();
	let restored: Uniform<char> = serde_json::from_str(&saved).unwrap();

	let mut rand1 = crate::seeded(42);
	let mut rand2 = crate::seeded(42);
	for _ in 0..100 {
		assert_eq!(rand1.sample(&chars), rand2.sample(&restored));
	}
}
