Performance comparison to the `rand` crate
==========================================

Comparison of rand's `SmallRng` vs urandom's `new()`
----------------------------------------------------

Both crates use the Xoshiro256 family of algorithms.

```
running 6 tests
test f64_rand           ... bench:         857.52 ns/iter (+/- 52.28) = 1194 MB/s
test f64_urandom        ... bench:         748.83 ns/iter (+/- 0.99) = 1368 MB/s
test fill_bytes_rand    ... bench:      92,316.10 ns/iter (+/- 66.24) = 11 MB/s
test fill_bytes_urandom ... bench:      79,287.00 ns/iter (+/- 112.22) = 12 MB/s
test u64_rand           ... bench:         763.68 ns/iter (+/- 15.58) = 1342 MB/s
test u64_urandom        ... bench:         687.31 ns/iter (+/- 4.58) = 1490 MB/s
```

There is no difference expected for fill_bytes and u64.

For f64 urandom uses a faster variant of xoshiro256 since only 53 bits are needed.

Comparison of rand's `StdRng` vs urandom's `csprng()`
-----------------------------------------------------

Both crates use the ChaCha12 algorithm.

```
running 6 tests
test f64_rand           ... bench:       2,002.38 ns/iter (+/- 65.92) = 511 MB/s
test f64_urandom        ... bench:       2,379.33 ns/iter (+/- 21.93) = 430 MB/s
test fill_bytes_rand    ... bench:     198,730.90 ns/iter (+/- 646.17) = 5 MB/s
test fill_bytes_urandom ... bench:     188,155.88 ns/iter (+/- 33,147.61) = 5 MB/s
test u64_rand           ... bench:       1,279.13 ns/iter (+/- 24.17) = 800 MB/s
test u64_urandom        ... bench:       1,313.37 ns/iter (+/- 11.98) = 779 MB/s
```

Comparison of rand's `UniformInt` vs urandom's `UniformInt`
-----------------------------------------------------------

```
running 4 tests
test uniform_range_rand     ... bench:       4,339.98 ns/iter (+/- 32.66)
test uniform_range_urandom  ... bench:         908.10 ns/iter (+/- 2.13)
test uniform_sample_rand    ... bench:       1,194.32 ns/iter (+/- 5.91)
test uniform_sample_urandom ... bench:         940.89 ns/iter (+/- 7.71)
```

Urandom uses a faster algorithm to sample from a range. This avoids an expensive integer division when constructing the range sampler.
