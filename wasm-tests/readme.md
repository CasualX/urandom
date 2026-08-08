# WebAssembly ChaCha tests

This standalone crate builds the same deterministic ChaCha test harness with scalar and SIMD128 backends.
The Node runner executes both modules, checks their output against a fixed fingerprint, and verifies that their streams match.

From this directory:

```console
CARGO_TARGET_DIR=target/scalar RUSTFLAGS="-C target-feature=-simd128" cargo build --release --target wasm32-unknown-unknown
CARGO_TARGET_DIR=target/simd RUSTFLAGS="-C target-feature=+simd128" cargo build --release --target wasm32-unknown-unknown
node run.mjs
```
