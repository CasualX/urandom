Frequently Asked Questions
--------------------------

This was written a while ago and the comparison with the `rand` crate may no longer be relevant.

Q: *Why another random number generator crate?*

A: Because I can do better than the standard rand crate's design.
   My random crate is simpler, easier to use and faster at runtime.

Q: *Which random number generators are implemented?*

A: `Xoshiro256` as PRNG by [Sebastiano Vigna and David Blackman](http://prng.di.unimi.it/) (supported by `SplitMix64` when seeding from `u64`). `ChaCha12` as CSPRNG by [Daniel J. Bernstein](https://cr.yp.to/chacha/chacha-20080128.pdf). [`getrandom`](https://crates.io/crates/getrandom) as the source of system entropy.

Q: *Why are random floats generated in the half-open interval `[1.0, 2.0)` instead of `[0.0, 1.0)`?*

A: Because it's easier and faster and it avoids hard (design and implementation) questions. Naively subtracting `1.0` leaves a bias in the low bits of the float's mantissa (see `examples/float_bias.rs`). The `Float01` distribution generates a random float in open interval `(0.0, 1.0)` without bias.

Q: *This is basically a copy and paste of `rand` with less features*

A: That is not a question. The distribution related structs and traits are fairly well designed and didn't need much change. Focus is on the PRNG itself, the `Rng` trait and the `Random` interface and the constructors.

Q: *Well then, what exactly is wrong with `rand`?*

A: A few things:

   The `rand` crate puts its `thread_rng` front and center as it's the easiest way to generate randomness (through explicit use or `random` method).
   Not a fan of thread local variables they suffer the same problem as global state except they're, well, thread safe. But global variables are still bad.
   This crate seeds new PRNG's directly from the system's getrandom.

   The `rand` crate requires importing a lot traits to make use of its functionality. Granted this is somewhat alleviated by the `prelude` module but not a fan. Rust IDE experience isn't there yet to make this smooth (eg. auto importing missing traits). The functionality is presented as inherent methods on the `Random` struct requiring no imports and smooth IDE experience.

Q: *How does this crate work without getrandom for seeding?*

A: When the opting out of the `getrandom` crate its functionality is deferred to a function named `getentropy_raw` with C linkage.
   Simply define this symbol as you would in C and it will be linked up as the secure source of entropy.

Q: *Is it performant on 32-bit systems?*

A: This crate is optimized for 64-bit architectures with fast full 64-bit integer multiplication in mind.
