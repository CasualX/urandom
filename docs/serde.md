# Serde data model

The optional `serde` feature serializes generator and distribution state.
It is available in `no_std` environments and does not require allocation.

## Quick overview

Urandom uses Serde's native data model rather than defining a byte format of its own.
The chosen Serde backend therefore determines what can be represented and what information is retained.

| Value | Serde representation | Backend requirement |
|---|---|---|
| `Random<R>` | Transparent to its concrete `R` | Whatever the concrete generator requires |
| `Uniform<T>` | Transparent to its concrete sampler | Whatever the sampler requires |
| Integer state | Native Serde integer | Support for wide integers if used |
| Floating-point state | Native Serde floating-point values | Support for infinities and nans if present |
| SplitMix state | Fixed structs containing integer state | Support unsigned 64-bit integers exactly |
| Xoshiro state | Fixed structs containing integer state | Support unsigned 64-bit integers exactly |
| ChaCha state | A named newtype containing a map with a dynamic set of entries | Newtypes, maps, string-like field identifiers, and fixed sequences |

For a quick rule of thumb, human-readable, self-describing formats with normal
map support are a natural fit. Compact or schema-driven formats should be
checked against the requirements above, especially before choosing them for
ChaCha snapshots.

## Distributions

Distribution parameters and sampler state use their ordinary Rust integer and floating-point types.
Integers are passed directly to Serde's signed or unsigned integer model,
and `f32` and `f64` values are passed directly to its floating-point model.
Urandom does not convert them to strings, byte strings, or an independently specified numeric encoding.

Consequently, backend limitations apply directly. Examples include unsupported
wide integers, rejection of non-finite floats, loss of a floating-point detail
in a text representation, or a numeric range imposed by another ecosystem.
Whether such values round-trip is a property of the backend as well as urandom's data model.

Deserialization restores the stored sampler state rather than reconstructing a
distribution from its public constructor arguments. This is what allows a
snapshot to preserve its behavior, but applications should treat untrusted
serialized distributions as data requiring validation.

## Generator state

Serde support is intended for deterministic generators whose state can be resumed.
`SplitMix64Rng` and `Xoshiro256Rng` have fixed, integer-based state.
`Random<R>` serializes exactly as the concrete `R`, so wrapping one of these
generators does not change its stored form.

The others do not serialize. Their state is either an external source of randomness
or an application-owned input rather than a portable deterministic snapshot.

### ChaCha

`ChaCha8Rng`, `ChaCha12Rng`, and `ChaCha20Rng` expose distinct newtype names to
Serde. A backend that preserves and checks newtype struct names can therefore
retain the round-count identity.

That identity is not guaranteed by Serde itself. Formats such as JSON ignore
newtype names, so their serialized ChaCha values do not identify the round count.
Deserialization still targets a concrete Rust type, but accidentally
reading a snapshot as another ChaCha round count may succeed and resume a
different stream. Applications using such a format should keep the concrete
type out of band or add their own tagged envelope.

ChaCha uses a map because its cached output block is conditional. A snapshot
always contains the core generator state. The cache position and cached output
are either both present or both absent; partial cache state is rejected.
Backends used for ChaCha must therefore support maps with a data-dependent set
of entries and field identifiers, in addition to fixed integer sequences.

Serialized ChaCha state contains the secret seed and may contain buffered
keystream. Anyone who obtains it can reproduce the stream. It must be protected
with the same care as the original seed.

## Compatibility policy

Urandom's [reproducibility policy](../readme.md#reproducibility-policy) covers
serialized deterministic generator state. State written by one release remains
readable by SemVer-compatible releases and resumes the same stream at the saved
position, provided the Serde backend itself round-trips the data model.

Snapshots do not contain a urandom schema version. The representation is stable
under the compatibility policy rather than migrated through an embedded version field.

## Remaining limitations

- Numeric portability is limited by the chosen backend. This is most visible
  for wide integers, non-finite floats, and text formats that do not preserve
  every floating-point distinction.
- ChaCha snapshots require dynamic map support, which makes them unsuitable for
  some compact or schema-driven Serde backends.
- Generator identity is backend-dependent. In particular, JSON does not retain
  the distinct ChaCha newtype names.
- Deserialization is primarily a snapshot-restoration interface. Not every
  internal invariant is revalidated as if the value had gone through a public
  constructor, so untrusted input should be validated or isolated by the
  application.
- The current test suite exercises JSON round-trips. Other backends are governed
  by the documented Serde data-model requirements but are not yet covered by
  interoperability tests.
