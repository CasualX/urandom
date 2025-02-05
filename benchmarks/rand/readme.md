Performance comparison to the `rand` crate
==========================================

Comparison of rand's `SmallRng` vs urandom's `new()`
----------------------------------------------------

Both crates use the Xoshiro256 family of algorithms.

```
running 6 tests
test f64_rand           ... bench:         858.54 ns/iter (+/- 28.51) = 1193 MB/s
test f64_urandom        ... bench:         762.32 ns/iter (+/- 17.60) = 1343 MB/s
test fill_bytes_rand    ... bench:      92,386.74 ns/iter (+/- 217.56) = 11 MB/s
test fill_bytes_urandom ... bench:      79,506.86 ns/iter (+/- 830.81) = 12 MB/s
test u64_rand           ... bench:         766.34 ns/iter (+/- 13.36) = 1336 MB/s
test u64_urandom        ... bench:         688.67 ns/iter (+/- 28.01) = 1488 MB/s
```

There is no difference expected for fill_bytes and u64.

For f64 urandom uses a faster variant of xoshiro256 since only 53 bits are needed.

Comparison of rand's `StdRng` vs urandom's `csprng()`
-----------------------------------------------------

Both crates use the ChaCha12 algorithm.

```
running 6 tests
test f64_rand           ... bench:       2,312.97 ns/iter (+/- 293.39) = 442 MB/s
test f64_urandom        ... bench:       2,977.51 ns/iter (+/- 63.71) = 343 MB/s
test fill_bytes_rand    ... bench:     181,497.00 ns/iter (+/- 10,974.79) = 5 MB/s
test fill_bytes_urandom ... bench:     187,261.64 ns/iter (+/- 19,665.52) = 5 MB/s
test u64_rand           ... bench:       1,264.85 ns/iter (+/- 9.01) = 810 MB/s
test u64_urandom        ... bench:       1,522.70 ns/iter (+/- 11.78) = 672 MB/s
```

Comparison of rand's `UniformInt` vs urandom's `UniformInt`
-----------------------------------------------------------

```
running 4 tests
test uniform_range_rand     ... bench:       1,040.08 ns/iter (+/- 30.73)
test uniform_range_urandom  ... bench:         907.35 ns/iter (+/- 5.27)
test uniform_sample_rand    ... bench:       1,035.46 ns/iter (+/- 11.28)
test uniform_sample_urandom ... bench:         948.45 ns/iter (+/- 10.77)
```

Urandom used to be much faster due to using a faster algorithm but rand has since switched to the same algorithm.
