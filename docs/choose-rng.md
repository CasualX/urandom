# Choosing a random number generator

Urandom includes a small, opinionated set of random number generators. The right
choice depends first on whether output must be unpredictable and then on how the
application creates and distributes random streams.

## Quick choice

| Requirement | Recommended choice |
|---|---|
| General application use, including security-sensitive output | [`urandom::new()`], which uses `ChaCha12Rng` |
| Reproducible general-purpose randomness | [`urandom::seeded(seed)`], which uses `Xoshiro256Rng` |
| Fast non-cryptographic randomness | [`Xoshiro256Rng`] |
| Recursive fork-join stream creation | [`SplittableRandom`] with [`fork`] |
| Direct output from the operating system entropy source | [`SystemRng`] |
| Greater or smaller ChaCha security margin | [`ChaCha20Rng`] or [`ChaCha8Rng`] |

If there is no specific reason to select a concrete generator, use `urandom::new()`.
If reproducibility is required and cryptographic unpredictability is not, use `urandom::seeded(seed)`.

## ChaCha generators

[`ChaCha8Rng`], [`ChaCha12Rng`], and [`ChaCha20Rng`] are deterministic, cryptographically secure generators.
The number in the name is the number of ChaCha rounds and represents a speed-versus-security-margin choice:

- `ChaCha12Rng` is the recommended balance and the generator returned by `urandom::new()`.
- `ChaCha8Rng` performs fewer rounds.
  Choose it when additional throughput matters and its smaller security margin has been deliberately accepted.
- `ChaCha20Rng` performs the full 20 rounds.
  Choose it when a conservative security margin matters more than the additional cost.

For normal security-sensitive use, prefer the root constructor:

```rust
let mut rand = urandom::new();
let token = rand.random_bytes::<32>();
```

The equivalent explicit construction is useful when a concrete type is needed:

```rust
use urandom::rng::{ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};

let fast = ChaCha8Rng::new();
let balanced = ChaCha12Rng::new();
let conservative = ChaCha20Rng::new();
```

These `new` constructors seed each generator from system entropy. A native 256-bit
seed supplied to `from_seed` can also be secure, but only when the seed is secret
and comes from a cryptographically secure source.

`from_seed_u64` is intended for reproducibility, not security. A public or
low-entropy seed does not become secret merely because ChaCha expands it into a
larger internal state:

```rust
use urandom::rng::ChaCha12Rng;

// Reproducible, but not unpredictable.
let mut a = ChaCha12Rng::from_seed_u64(42);
let mut b = ChaCha12Rng::from_seed_u64(42);
assert_eq!(a.random::<u64>(), b.random::<u64>());
```

ChaCha generators also support `split` and `fork`. Use `split` when one coordinator
creates a flat collection of streams. Use `fork` when both descendants may divide
again; ChaCha derives two independently keyed descendants from generated seed
material. This is more expensive than SplittableRandom's specialized fork and is
appropriate when the descendants must remain cryptographically secure.

## Xoshiro256Rng

`Xoshiro256Rng` is the recommended general-purpose generator after cryptographic
unpredictability has been ruled out. Typical uses include simulations, games,
procedural generation, randomized algorithms, sampling, shuffling, and tests. It
has a 256-bit state and a period of 2<sup>256</sup> - 1.

Use `new` when each run should start from fresh system entropy:

```rust
use urandom::rng::Xoshiro256Rng;

let mut rand = Xoshiro256Rng::new();
let value: u64 = rand.random();
```

Use `urandom::seeded` or `from_seed_u64` when the run must be reproducible:

```rust
use urandom::rng::Xoshiro256Rng;

let mut a = urandom::seeded(42);
let mut b = Xoshiro256Rng::from_seed_u64(42);

assert_eq!(a.random::<u64>(), b.random::<u64>());
```

The root-level `seeded` constructor returns `Random<Xoshiro256Rng>`, making it the
most concise expression of the recommended deterministic default.

### Flat streams with split

When one coordinator creates a flat collection of streams, use [`split`]. It
returns the generator at its current position and advances the coordinator by
2<sup>128</sup> steps. This partitions Xoshiro's cycle into non-overlapping
subsequences, provided no worker consumes more than 2<sup>128</sup> values:

```rust
use urandom::rng::Xoshiro256Rng;

let mut coordinator = Xoshiro256Rng::from_seed_u64(42);
let worker_0 = coordinator.split();
let worker_1 = coordinator.split();
let worker_2 = coordinator.split();
```

Only call `split` on the coordinator. Calling it on one of the returned generators
can reproduce a stream that the coordinator will later return.

### Recursive streams with fork

Xoshiro256 can also be forked. Its `fork` derives two new 256-bit states from
separately mixed output material. This is useful when recursive forking is
occasional and retaining Xoshiro is convenient.

This derivation is not the same as Xoshiro's jump operation: the resulting states
are not a mathematical partition of the original cycle. It is also more expensive
than `SplittableRandom::fork` because it constructs two complete 256-bit states.
Prefer SplittableRandom when fork throughput is central to the workload.

## SplittableRandom

`SplittableRandom` is a specialized non-cryptographic generator for fork-join
programs that repeatedly divide work and give each branch its own random stream.
Its state consists of a 64-bit position and an odd 64-bit stream parameter. Each
individual stream has a period of 2<sup>64</sup>.

Create an entropy-seeded or reproducible root as needed:

```rust
use urandom::rng::SplittableRandom;

let fresh = SplittableRandom::new();
let reproducible = SplittableRandom::from_seed_u64(42);
```

Use `fork` whenever a task divides. It consumes the current generator and returns
two descendants; either descendant may be forked again:

```rust
use urandom::rng::SplittableRandom;

let root = SplittableRandom::from_seed_u64(42);
let (left, right) = root.fork();
let (left_left, left_right) = left.fork();
let (right_left, right_right) = right.fork();
```

This ownership pattern makes it natural to move one generator into each worker
without sharing or synchronization. Given the same root seed and tree of `fork`
calls, the descendants are reproducible. For stable results, assign tree positions
to jobs deterministically instead of letting thread scheduling choose which worker
receives each descendant.

SplittableRandom's fork is considerably cheaper than Xoshiro256's fork.
Xoshiro's absolute fork cost is still small when each descendant performs meaningful work.
SplittableRandom matters most for very fine-grained workloads or programs that create millions of recursive streams.

## SystemRng

`SystemRng<N>` bypasses deterministic PRNG generation and obtains random data from
the operating system through `getrandom`. Choose it when values should come directly
from the system entropy source and reproducibility, deterministic stream management,
and PRNG throughput are not wanted.

```rust
use urandom::rng::SystemRng;

let mut rand = SystemRng::<64>::new();
let value: u64 = rand.random();
let bytes = rand.random_bytes::<32>();
```

For `u32` and `u64` generation, `N` is the number of 32-bit words fetched and
buffered at once. It must be at least 2, and 64 or more is recommended to amortize
calls to the operating system. Byte-filling methods bypass that word buffer and
request the destination bytes directly from the system source.

SystemRng is normally much slower than expanding a seed with ChaCha, Xoshiro, or
SplittableRandom. It also cannot reproduce a run, be serialized as deterministic
state, or create deterministic child streams. Use ChaCha12 when the normal goal is
secure application randomness; use SystemRng specifically when direct system output
is the desired behavior.

`SystemRng` and entropy-seeded `new` constructors require the default `getrandom`
feature. Deterministic constructors remain available when that feature is disabled.

## Reproducibility and stream ownership

The generator algorithm, seed, and sequence of operations all form part of a
reproducible result. Switching generators changes the stream. So can replacing one
`u64` draw with two `u32` draws, changing the shape of a fork tree, or assigning
descendants to jobs in a different order.

Prefer owned generators over one shared generator in parallel code. Give each worker
its own result from `split` or `fork`; this avoids synchronization and makes random
state consumption explicit.

Do not treat deterministic construction as secure seeding. In particular,
`from_seed_u64(42)` remains predictable for every generator, including ChaCha20.

[`ChaCha8Rng`]: https://docs.rs/urandom/latest/urandom/rng/type.ChaCha8Rng.html
[`ChaCha12Rng`]: https://docs.rs/urandom/latest/urandom/rng/type.ChaCha12Rng.html
[`ChaCha20Rng`]: https://docs.rs/urandom/latest/urandom/rng/type.ChaCha20Rng.html
[`fork`]: https://docs.rs/urandom/latest/urandom/struct.Random.html#method.fork
[`split`]: https://docs.rs/urandom/latest/urandom/struct.Random.html#method.split
[`SplittableRandom`]: https://docs.rs/urandom/latest/urandom/rng/struct.SplittableRandom.html
[`SystemRng`]: https://docs.rs/urandom/latest/urandom/rng/struct.SystemRng.html
[`Xoshiro256Rng`]: https://docs.rs/urandom/latest/urandom/rng/struct.Xoshiro256Rng.html
[`urandom::new()`]: https://docs.rs/urandom/latest/urandom/fn.new.html
[`urandom::seeded(seed)`]: https://docs.rs/urandom/latest/urandom/fn.seeded.html
