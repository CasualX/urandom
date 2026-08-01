# Serde data model

The optional `serde` feature serializes generator and distribution state.
It is available in `no_std` environments and does not require allocation.

## Quick overview

Urandom uses Serde's native data model.
Correct serialization depends on the backend support:

| Value | Representation | Backend requirement |
|---|---|---|
| Integer state | Native Serde integer | Support for wide integers if used |
| Floating-point state | Native Serde floating-point values | Support for infinities and NaNs if present |
| Xoshiro state | Fixed structs containing integer state | Support unsigned 64-bit integers exactly |
| ChaCha state | A named newtype containing a map with a dynamic set of entries | Newtypes, maps, string-like field identifiers, and fixed sequences |

## Distributions

Distribution parameters and sampler state use ordinary Rust integer and
floating-point types, which are passed directly to Serde's data model.

Consequently, backend limitations apply directly. Examples include unsupported
wide integers, rejection of non-finite floats, loss of floating-point precision
in a text representation, or a numeric range imposed by another ecosystem.
Whether such values round-trip is a property of the backend.

Deserialization restores stored sampler state directly instead of reconstructing
the distribution from its public constructor arguments.

## Generator state

Serde support is implemented for deterministic generators whose state can be resumed:
`Xoshiro256Rng`, `ChaCha8Rng`, `ChaCha12Rng`, and `ChaCha20Rng`.
The others do not serialize.

### ChaCha

`ChaCha8Rng`, `ChaCha12Rng`, and `ChaCha20Rng` expose distinct newtype names to
Serde. A backend that preserves and checks newtype struct names can therefore
retain the round-count identity.

That identity is not guaranteed by Serde itself. Formats such as JSON ignore
newtype names, so their serialized ChaCha values do not identify the round count.
Deserialization still targets a concrete Rust type, but accidentally
reading a snapshot as another ChaCha round count may succeed and resume a
different stream.

Serialized ChaCha state contains the secret seed and buffered keystream.
Anyone who obtains it can reproduce the stream. It must be protected
with the same care as the original seed.

## Compatibility policy

Urandom's [reproducibility policy](../readme.md#reproducibility-policy) covers
serialized deterministic generator state. State written by one release remains
readable by SemVer-compatible releases and resumes the same stream at the saved
position, provided the Serde backend itself round-trips the data model.

Snapshots do not include a schema version; compatibility follows the policy above.

## `serde_json`

The Serde tests currently use `serde_json`. Its text serializer and deserializer
preserve the full `i128` and `u128` ranges, although other JSON implementations
may reject these values or lose precision. Without the `arbitrary_precision`
feature, integer values below `i64::MIN` or above `u64::MAX` cannot pass through
`serde_json::Value`.

`serde_json` serializes NaN and positive or negative infinity as `null`.
Deserializing that output into a floating-point field fails, so non-finite
values do not round-trip.
