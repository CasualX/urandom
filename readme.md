µrandom
=======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/urandom.svg)](https://crates.io/crates/urandom)
[![docs.rs](https://docs.rs/urandom/badge.svg)](https://docs.rs/urandom)

Produce and consume randomness.

This crate provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms.

# Quick Start

To get you started quickly, the easiest and highest-level way to get a random value is to use `urandom::new().next()`.
The `Random` struct provides a useful API on all Rngs, while the `distributions` module provide further functionality on top of Rngs.

```rust
let mut rng = urandom::new();

// Generates a random boolean
if rng.coin_flip() {
	// Try printing a random unicode code point (probably a bad idea)!
	println!("char: {}", rng.next::<char>());
}

// Generates a float between 13.0 and 42.0
let y: f64 = rng.range(13.0..42.0);

// Shuffles the list of numbers
let mut numbers: Vec<i32> = (1..100).collect();
rng.shuffle(&mut numbers);
```

This library was inspired by the semi-official [`rand`](https://crates.io/crates/rand) crate and an attempt to provide a better experience.

Frequently Asked Questions
--------------------------

Q: *Why another random number generator crate?*

A: Because I think I can do better than the standard rand crate's design.
   My random crate is simpler, easier to use and faster at runtime.

Q: *Which random number generators are implemented?*

A: `Xoshiro256` as PRNG by [Sebastiano Vigna and David Blackman](http://prng.di.unimi.it/) (supported by `SplitMix64` when seeding from `u64`). `ChaCha20` as CSPRNG by [Daniel J. Bernstein](http://loup-vaillant.fr/tutorials/chacha20-design). `getrandom` as the source of system entropy.

Q: *Why are random floats generated in the half-open interval `[1.0, 2.0)` instead of `[0.0, 1.0)`?*

A: Because it's easier and faster and it avoids hard (design and implementation) questions. Naively subtracting `1.0` leaves a bias in the low bits of the float's mantissa (see [`examples/float_bias.rs`]). The `Float01` distribution generates a random float in open interval `(0.0, 1.0)` without bias.

Q: *This is basically a copy and paste of `rand` with less features*

A: That is not a question. The distribution related structs and traits are fairly well designed and didn't need much change. I focussed on the PRNG itself, the `Rng` trait and the `Random` interface and the constructors.

Q: *Well then, what exactly is wrong with `rand`?*

A: A few things stood out to me. I'm not a fan of thread local variables they suffer the same problem as global state except they're, well, thread safe. But global variables are still bad.

   The `rand` crate puts its `thread_rng` front and center as it's the easiest way to generate randomness (through explicit use or `random` method). I seed new PRNG's directly from the system's getrandom.

   The `rand` crate requires importing a lot traits to make use of its functionality. Granted this is somewhat alleviated by the `prelude` module but I'm not a fan. Rust IDE experience isn't there yet to make this smooth (eg. auto importing missing traits). I put the functionality as inherent methods on the `Random` struct requiring no imports and smooth IDE experience.

   The `rand` crate tries to abstract too much eg. the `CryptoRng` trait and related functionality. These help _implementing_ the necessary abstractions to adapt them for use as a PRNG.

Q: *How does this crate work without getrandom for seeding?*

A: When the opting out of the `getrandom` crate its functionality is deferred to a function named `getentropy_raw` with C linkage.
   Simply define this symbol as you would in C and it will be linked up as the secure source of entropy.

Q: *Is it performant on 32-bit systems?*

A: I optimized this crate for 64-bit architectures with fast full 64-bit integer multiplication in mind.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
