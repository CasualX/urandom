use super::*;

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
		let mut low: i16 = rand.next();
		let mut high: i16 = rand.next();
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
