Performance comparison to the `rand` crate
==========================================

These benchmarks compare the current urandom 1.0 implementation with rand
0.10.2. The results below were collected on my machine using rustc 1.98.0-nightly
(2026-06-01, LLVM 22.1.6). The benchmark process was pinned to one core.

The exact timings are machine- and compiler-dependent. The conclusions from
the generated assembly are more important than small differences in the numbers.

Comparison of rand's `SmallRng` vs urandom's `Xoshiro256Rng`
------------------------------------------------------------

Both crates use xoshiro256++ for `u64` generation on 64-bit targets.

```
running 8 tests
test f64_rand           ... bench:       1,032.61 ns/iter (+/- 4.73) = 7751 MB/s
test f64_urandom        ... bench:         787.69 ns/iter (+/- 64.72) = 10165 MB/s
test fill_bytes_rand    ... bench:      79,634.09 ns/iter (+/- 2,227.19) = 12858 MB/s
test fill_bytes_urandom ... bench:      79,340.60 ns/iter (+/- 266.78) = 12906 MB/s
test u32_rand           ... bench:         835.56 ns/iter (+/- 4.78) = 4790 MB/s
test u32_urandom        ... bench:         787.75 ns/iter (+/- 9.33) = 5082 MB/s
test u64_rand           ... bench:         814.15 ns/iter (+/- 3.49) = 9828 MB/s
test u64_urandom        ... bench:         814.27 ns/iter (+/- 19.64) = 9828 MB/s
```

The optimized `u64` hot loops contain the same state transition and have the
same performance. Small apparent wins in either direction disappear across
repeated pinned runs.

Urandom's `u32` and `f64` paths use xoshiro256+ because those values do not
need all 64 output bits. Rand generates a full xoshiro256++ `u64` first.
This contributes to urandom's advantage in those two benchmarks.

Comparison of rand's `StdRng` vs urandom's `ChaCha12Rng`
--------------------------------------------------------

Both crates use ChaCha12 and buffer four 64-byte blocks per refill.

Urandom selects its ChaCha backend with `target_feature` configuration at compile time.
Rand's `chacha20` dependency performs runtime CPU detection and selects its AVX2
implementation on this machine. Benchmarked with `RUSTFLAGS="-C target-cpu=native"`.

```
running 8 tests
test f64_rand           ... bench:       2,198.65 ns/iter (+/- 11.24) = 3639 MB/s
test f64_urandom        ... bench:       2,010.85 ns/iter (+/- 12.66) = 3980 MB/s
test fill_bytes_rand    ... bench:     203,604.55 ns/iter (+/- 29,142.81) = 5029 MB/s
test fill_bytes_urandom ... bench:     211,407.42 ns/iter (+/- 10,072.18) = 4843 MB/s
test u32_rand           ... bench:         984.90 ns/iter (+/- 7.48) = 4065 MB/s
test u32_urandom        ... bench:       1,031.20 ns/iter (+/- 8.86) = 3879 MB/s
test u64_rand           ... bench:       1,681.83 ns/iter (+/- 14.05) = 4759 MB/s
test u64_urandom        ... bench:       1,671.55 ns/iter (+/- 10.74) = 4787 MB/s
```

Comparison of rand's `UniformInt` vs urandom's `UniformInt`
-----------------------------------------------------------

```
running 4 tests
test uniform_range_rand     ... bench:       1,079.31 ns/iter (+/- 8.46)
test uniform_range_urandom  ... bench:         941.54 ns/iter (+/- 48.85)
test uniform_sample_rand    ... bench:       1,097.90 ns/iter (+/- 35.66)
test uniform_sample_urandom ... bench:         950.26 ns/iter (+/- 41.07)
```

The two rand rows take different paths. A constructed `Uniform` uses unbiased
Lemire sampling; its `UniformInt` computes and stores the rejection threshold
during construction. `random_range` instead dispatches through
`UniformSampler::sample_single`. With rand's default features, that method uses
Canon's method, which rand documents as slightly biased; enabling rand's
optional `unbiased` feature selects a looping variant instead.

Urandom has no separate single-sample hook. Its `UniformInt` implements one
unbiased multiply-and-reject path for both uses and computes the exact threshold
lazily, usually returning before the expensive modulo is needed. This explains
its advantage in these default-feature benchmarks while avoiding the bias of
rand's one-off path.
