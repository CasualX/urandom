#[cfg(feature = "getrandom")]
#[test]
fn test_choose() {
	let mut rand = crate::new();

	let mut array = [0, 1, 2, 3, 4];
	let mut result = [0i32; 5];

	for _ in 0..10000 {
		result[*rand.choose(&array).unwrap()] += 1;
		result[*rand.choose_mut(&mut array).unwrap()] += 1;
	}

	let mean = (result[0] + result[1] + result[2] + result[3] + result[4]) / 5;
	let success = result.iter().all(|&x| (x - mean).abs() < 500);
	assert!(success, "mean: {mean}, result: {result:?}");
}

#[cfg(feature = "getrandom")]
#[test]
fn weighted_index_supports_floating_weights() {
	let mut rand = crate::new();
	let weights = [1.0, 2.0, 7.0];
	let mut counts = [0i32; 3];

	for _ in 0..100_000 {
		counts[rand.weighted_index(weights.len(), 10.0, |index| weights[index]).unwrap()] += 1;
	}

	let expected = [10_000, 20_000, 70_000];
	let success = counts.iter().zip(expected).all(|(&count, expected)| (count - expected).abs() < 1_000);
	assert!(success, "expected: {expected:?}, counts: {counts:?}");
}

#[test]
fn weighted_index_uses_one_random_value() {
	let mut rand = crate::rng::MockRng::slice(&[0, 42]);
	let weights = [1.0, 2.0, 7.0];

	assert_eq!(rand.weighted_index(weights.len(), 10.0, |index| weights[index]), Some(0));
	assert_eq!(rand.random::<u64>(), 42);
}

#[test]
fn weighted_index_returns_none_without_a_sample_space() {
	let mut rand = crate::seeded(42);

	assert_eq!(rand.weighted_index(0, 1.0, |_| 1.0), None);
	assert_eq!(rand.weighted_index(1, 0.0, |_| 1.0), None);
	assert_eq!(rand.weighted_index(1, f64::NAN, |_| 1.0), None);
	assert_eq!(rand.weighted_index(3, 1.0, |_| 0.0), None);
}

#[test]
fn weighted_index_skips_non_positive_floating_weights() {
	let weights = [0.0, -1.0, f64::NAN, 10.0];
	let mut rand = crate::rng::MockRng::slice(&[0]);

	assert_eq!(rand.weighted_index(weights.len(), 10.0, |index| weights[index]), Some(3));
}

#[test]
fn weighted_index_accepts_fn_mut() {
	let mut calls = 0;
	let weights = [10u32];
	let mut rand = crate::rng::MockRng::slice(&[u64::MAX]);
	let selected = rand.weighted_index(weights.len(), 10, |index| {
		calls += 1;
		weights[index]
	});

	assert_eq!(selected, Some(0));
	assert_eq!(calls, 1);
}

#[test]
fn choose_weighted_maps_elements_to_weights() {
	let items = [("disabled", 0i32), ("invalid", -1), ("selected", 10)];
	let mut calls = 0;
	let mut rand = crate::rng::MockRng::slice(&[u64::MAX]);

	let selected = rand.choose_weighted(&items, 10, |item| {
		calls += 1;
		item.1
	});

	assert_eq!(selected, Some(&items[2]));
	assert_eq!(calls, 3);
}

#[cfg(feature = "getrandom")]
#[test]
fn weighted_index_supports_integer_weights() {
	let mut rand = crate::new();
	let weights = [1u32, 2, 7];
	let mut counts = [0i32; 3];

	for _ in 0..100_000 {
		let index = rand.weighted_index(weights.len(), 10, |index| weights[index]).unwrap();
		counts[index] += 1;
	}

	let expected = [10_000, 20_000, 70_000];
	let success = counts.iter().zip(expected).all(|(&count, expected)| (count - expected).abs() < 1_000);
	assert!(success, "expected: {expected:?}, counts: {counts:?}");
}

#[test]
fn weighted_index_skips_non_positive_integer_weights() {
	let weights = [0i32, -1, 10];
	let mut rand = crate::rng::MockRng::slice(&[u64::MAX]);

	assert_eq!(rand.weighted_index(weights.len(), 10, |index| weights[index]), Some(2));
}

#[test]
fn weighted_index_supports_an_unused_remainder() {
	let mut rand = crate::rng::MockRng::slice(&[u64::MAX]);
	assert_eq!(rand.weighted_index(3, 10u32, |_| 1), None);
}

#[cfg(feature = "getrandom")]
#[test]
fn test_choose_iter_reservoir() {
	let unknown_size = || (0..4).filter(|_| true);
	let mut rand = crate::new();
	let mut counts = [0i32; 4];

	for _ in 0..10000 {
		counts[rand.choose_iter(unknown_size()).unwrap()] += 1;
	}

	let mean = counts.iter().sum::<i32>() / counts.len() as i32;
	let success = counts.iter().all(|&count| (count - mean).abs() < 500);
	assert!(success, "mean: {mean}, counts: {counts:?}");

	assert_eq!(rand.choose_iter(core::iter::empty::<i32>()), None);
}

#[cfg(feature = "getrandom")]
#[test]
fn test_choose_multiple_reservoir_range() {
	let mut rand = crate::new();
	let mut counts = [0i32; 2];

	for _ in 0..10000 {
		let mut result = [0];
		assert_eq!(rand.choose_multiple(0..2, &mut result), 1);
		counts[result[0]] += 1;
	}

	let mean = (counts[0] + counts[1]) / 2;
	let success = counts.iter().all(|&count| (count - mean).abs() < 500);
	assert!(success, "mean: {mean}, counts: {counts:?}");
}

#[cfg(feature = "getrandom")]
#[test]
fn test_partial_shuffle() {
	let mut items = [1, 2, 3, 4, 100];
	let mut rng = crate::new();
	let items = rng.partial_shuffle(&mut items, 5);
	assert_eq!(items.len(), 5);
}
