# Migrating from 0.2.2 to 1.0

Version 1.0 requires Rust 1.95 or newer.

Update the dependency first:

```toml
[dependencies]
urandom = "1.0"
```

## Rename consumer methods

Most application code can be migrated with these replacements:

| 0.2.2 | 1.0 |
|---|---|
| `rand.next::<T>()`  | `rand.random::<T>()` |
| `rand.range(a..b)`  | `rand.uniform(a..b)` |
| `rand.single(iter)` | `rand.choose_iter(iter)` |
| `rand.multiple(iter, buf)` | `rand.choose_multiple(iter, buf)` |
| `rand.next_u32()` | `rand.random::<u32>()` |
| `rand.next_u64()` | `rand.random::<u64>()` |
| `rand.next_f32()` | `rand.uniform(1_f32..2_f32)` |
| `rand.next_f64()` | `rand.uniform(1_f64..2_f64)` |

`Random<R>` no longer implements `std::io::Read`; use `fill_bytes` instead:

```rust
let mut bytes = [0_u8; 32];
rand.fill_bytes(&mut bytes);
```

The low-level operations were removed from the `Random` struct.
If exact low-level calls are needed, import `urandom::rng::{Rng, JumpRng}`; `Random<R>` dereferences to its inner generator.

## Rename concrete generators

All public generator names now end in `Rng`:

| 0.2.2 | 1.0 |
|---|---|
| `Xoshiro256` | `Xoshiro256Rng` |
| `SplitMix64` | `SplitMix64Rng` |
| `ChaCha<N>` | `ChaChaRng<N>` |
| `ChaCha8` | `ChaCha8Rng` |
| `ChaCha12` | `ChaCha12Rng` |
| `ChaCha20` | `ChaCha20Rng` |
| `System<N>` | `SystemRng<N>` |
| `Mock<I>` | `MockRng<I>` |
| `Read<R>` | `ReadRng<R>` |

`Wyrand` was removed; use `Xoshiro256Rng` for non-cryptographic use.
`SystemRng<N>` now requires `N >= 2`.

The old 64-bit seed constructors for Xoshiro and ChaCha are now named `from_seed_u64`;
`from_seed` accepts the generator's native seed:

```rust
use urandom::rng::{ChaCha12Rng, Xoshiro256Rng};

let fast = Xoshiro256Rng::from_seed_u64(42);
let chacha = ChaCha12Rng::from_seed_u64(42);

let fast_native = Xoshiro256Rng::from_seed([1, 2, 3, 4]);
let chacha_native = ChaCha12Rng::from_seed([1, 2, 3, 4, 5, 6, 7, 8]);
```

A 64-bit seed does not make ChaCha output cryptographically unpredictable.
Use `ChaCha12Rng::new()` or `urandom::new()` when unpredictability matters.

## Use the secure root constructor

The root-level `csprng()` helper was renamed to `new()`. The old root-level `new()`, which
constructed a non-cryptographic Xoshiro256 generator, was removed. Code which needs that
generator should select it explicitly:

```rust
let mut secure = urandom::new();
let mut fast = urandom::rng::Xoshiro256Rng::new();
```

## Review changed behavior

- `random::<f32>()` and `random::<f64>()` now return `[0, 1)`, not `[1, 2)`.
- `choose_iter` and `choose_multiple` now perform unbiased integer-based reservoir sampling.
- `partial_shuffle(slice, n)` now returns the selected prefix.
- Bernoulli sampling now uses a strict `sample < p` boundary.
- `Exp::try_new(-0.0)` now returns `ExpError::LambdaTooSmall`.
- Floating-point uniform sampling, `Float01`, normal and exponential sampling, and `LogNormal::from_mean_cv` were corrected.
- `UniformError`, `ExpError`, and `NormalError` are now non-exhaustive; add a wildcard arm when matching them.

## Custom RNGs, state, and features

`Rng` is now sealed, so downstream crates cannot implement it.

`Random<R>` now derives `Debug` from `R` instead of using an opaque `Random(impl Rng)` representation.
ChaCha's implementation deliberately redacts its secret state.

Do not assume serialized 0.2.2 generators or distributions can be restored by 1.0.
The 1.x compatibility guarantee begins with 1.0 and covers supported deterministic generator state across SemVer-compatible releases.
See the [Serde data model guide](serde.md) for the supported generators and backend requirements.

The optional `getrandom` feature now uses `getrandom` 0.3 and is enabled by default.
It gates `urandom::new`, entropy-seeded generators' `new` constructors, `SystemRng`, `getentropy`, and `getentropy_uninit`.
Disable default features when only deterministic construction is needed.

The old `getentropy_raw` link-time integration used without `getrandom` was removed.
Custom system-entropy integrations must use `getrandom`'s custom-backend mechanism.

The `serde` feature is available without `std`.
