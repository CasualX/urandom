# Why I built urandom

I started urandom as a fork of [`rand`] because using randomness in Rust feels
off, its API difficult to discover over many crates and hidden state in threads.

I want one obvious object with methods for discovering randomness and its utilities.
I also wanted random state to be fully owned by the caller, no hidden thread-local state.
Those preferences shaped urandom to what it is today.

This is not an attempt to reproduce everything in the Rust Random ecosystem.
It is my opinionated alternative: a smaller API and a fixed set of generators.

I did keep the parts of rand I liked. Its distribution model was already well designed,
so urandom retains that approach rather than inventing a new abstraction.

The examples and details below compare `urandom` 1.0 with `rand` 0.10.2.
urandom requires Rust 1.95 or newer, while rand requires Rust 1.85 or newer.

## One place to look

Here is a small example of use of rand:

```rust
use rand::prelude::*;

let mut rng = rand::rng();

let roll: u32 = rng.random_range(1..=6);
let colors = ["red", "green", "blue"];
let color = colors.choose(&mut rng).unwrap();

let mut numbers: Vec<_> = (1..=10).collect();
numbers.shuffle(&mut rng);
```

The code is fine once you know the API.
My frustration is that its methods come from traits implemented on different types.
An operation may be on the RNG, slice, or iterator, and it only appears when the right trait is in scope.
The prelude makes the imports shorter, but it does not make the design easier to discover.

In urandom, the same operations live on `Random`:

```rust
let mut rng = urandom::new();
// rng: urandom::Random<urandom::rng::ChaCha12Rng>

let roll: u32 = rng.uniform(1..=6);
let colors = ["red", "green", "blue"];
let color = rng.choose(&colors).unwrap();

let mut numbers: Vec<_> = (1..=10).collect();
rng.shuffle(&mut numbers);
```

If you have a `Random`, IDE completion shows the useful consumer API:
`random`, `uniform`, `chance`, `choose`, `shuffle`, `sample`, and the rest.
You do not need to know which extension trait owns the operation before you can find it.
This is the main reason urandom exists.

## Random state should be visible

`rand::rng()` returns a handle to a thread-local ChaCha12 generator. It is a
convenient and cryptographically secure default, but the state itself is ambient:
I have a general aversion to hidden mutable state and I prefer explicitness.

`urandom::new()` creates a newly seeded ChaCha12 generator owned by the caller.
Select Xoshiro256 explicitly when cryptographic security is unnecessary and its
performance is useful:

```rust
let mut secure = urandom::new();
let mut fast = urandom::rng::Xoshiro256Rng::new();
```

With the default `getrandom` feature, both are seeded from the operating
system through the [`getrandom`] crate.

I prefer this ownership model because dependencies and consumption order remain
visible in the program. Passing the generator, storing it in a struct, or
creating an independent stream uses normal Rust values rather than access to
thread-local state.

Rand also has owned `SmallRng` and `StdRng`, named Xoshiro and ChaCha generators,
and a direct system source. It gives you the illusion of more choices;
urandom makes the common choice direct and more visible.

## Smaller contract

Rand's low-level traits connect a large ecosystem. A library can accept a
generic rand RNG, and callers can supply generators from other crates.
[`rand_distr`] adds a broad distribution catalog on the same interface.
That creates a large sprawling graph of randomness crates in your dependencies.

Urandom's `Rng` trait is sealed. You cannot plug an arbitrary third-party
generator into `Random`. That limitation lets me design the
generator and consumer sides together, keep the public contract small, and
in return optimize for my use case.

Its RNG traits are intentionally incompatible with rand. They exist to support
the generators supplied by urandom, not as another ecosystem-wide interface.
Keeping the generators, distributions, and sampling algorithms together also
makes the implementation easier to maintain.

The supplied generators cover the uses I care about:

- Xoshiro256: Very fast for non-cryptographic use.
- ChaCha8/12/20: Secure for cryptographic use.

They are well-known algorithms recommended today for their use cases, which is why I
do not see much value in exposing the generator as a consumer extension point.
Applications can still define their own distributions.

Urandom includes uniform, normal, log-normal, exponential, triangular, and Bernoulli distributions
plus sampling algorithms. Rand plus `rand_distr` has far more.
If your application needs those facilities, using rand is
more sensible than pretending the smaller scope is an advantage.

## Stronger reproducibility

My primary use case is a seeded generator for deterministic video game replays.
Urandom treats the output of its concrete deterministic generators as stable
across SemVer-compatible releases, provided the seed or state and the exact
sequence of low-level calls are the same.

Higher-level distributions and sampling algorithms have a weaker guarantee:
their behavior is preserved on a best-effort basis within a minor release and
may change in a new minor release. Pin a minor release when those results are
part of a recording.

Rand has its own stability policy, but it falls short of my use case:
SmallRng is not stable across 32-bit and 64-bit targets:

- They use different generators
- `usize` and `isize` use different distributions

With the `serde` feature, urandom can save and restore the state of its supported deterministic generators.
That representation is covered by the same compatibility policy; see the [Serde data model].

## Performance improvements

Urandom is faster in several common operations from the repository's [benchmarks].
On my system, compared with rand 0.10.2:

- Xoshiro `f64` throughput was about 14% higher and `u32` throughput about 9% higher.
  `u64` generation and bulk byte filling were effectively tied.
- Repeated one-off integer range sampling was about 19% faster. Reusable
  uniform sampling was effectively tied in this run. Urandom uses one unbiased
  multiply-and-reject implementation for both paths.
- ChaCha12 `f64` and `u64` throughput were effectively tied. Rand was about 5%
  faster for `u32` and about 8% faster for bulk byte filling.

These are real wins, not a claim that urandom is better in every benchmark.
CPU, compiler, target features, and workload all matter.

Urandom is optimized for 64-bit systems with fast full-width multiplication.
It uses the same generators on 32-bit systems for full compatibility, so those targets pay a performance cost.

## Try urandom

If you also want random state to be explicit, common operations to be easy to find,
and a crate focused on the consumer experience, give urandom a try.
Those are the choices I want in my own projects and the reason I continue to build it.
I hope it makes randomness a small, well-shaped part of your program.

[`rand`]: https://docs.rs/rand/0.10.2/rand/
[`rand_distr`]: https://docs.rs/rand_distr/latest/rand_distr/
[`getrandom`]: https://crates.io/crates/getrandom
[benchmarks]: benchmarks-rand.md
[Serde data model]: serde.md
