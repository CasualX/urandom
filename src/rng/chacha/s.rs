use super::*;
use crate::rng::{BlockRngImpl, SecureRng};

// These private newtypes give serializers the concrete round-count identity while preserving
// the inner representation for formats, such as JSON, which ignore newtype struct names.
#[derive(serde::Serialize, serde::Deserialize)]
struct ChaCha8Rng<T>(T);

#[derive(serde::Serialize, serde::Deserialize)]
struct ChaCha12Rng<T>(T);

#[derive(serde::Serialize, serde::Deserialize)]
struct ChaCha20Rng<T>(T);

impl<const N: usize> serde::Serialize for ChaChaRng<N>
where
	Self: SecureRng,
{
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match N {
			8 => serde::Serialize::serialize(&ChaCha8Rng(&self.inner), serializer),
			12 => serde::Serialize::serialize(&ChaCha12Rng(&self.inner), serializer),
			20 => serde::Serialize::serialize(&ChaCha20Rng(&self.inner), serializer),
			_ => unreachable!(),
		}
	}
}

impl<'de, const N: usize> serde::Deserialize<'de> for ChaChaRng<N>
where
	Self: SecureRng,
{
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let inner = match N {
			8 => <ChaCha8Rng<BlockRngImpl<ChaChaState<N>>>>::deserialize(deserializer)?.0,
			12 => <ChaCha12Rng<BlockRngImpl<ChaChaState<N>>>>::deserialize(deserializer)?.0,
			20 => <ChaCha20Rng<BlockRngImpl<ChaChaState<N>>>>::deserialize(deserializer)?.0,
			_ => unreachable!(),
		};
		Ok(ChaChaRng { inner })
	}
}

impl<const N: usize> serde::Serialize for ChaChaState<N> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		[
			self.seed[0], self.seed[1], self.seed[2], self.seed[3],
			self.seed[4], self.seed[5], self.seed[6], self.seed[7],
			self.counter[0], self.counter[1], self.stream[0], self.stream[1],
		].serialize(serializer)
	}
}

impl<'de, const N: usize> serde::Deserialize<'de> for ChaChaState<N> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let values = <[u32; 12]>::deserialize(deserializer)?;
		Ok(ChaChaState {
			seed: [values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7]],
			counter: [values[8], values[9]],
			stream: [values[10], values[11]],
		})
	}
}

impl serde::Serialize for ChaChaOutput {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		#[cfg(target_endian = "little")]
		return serde::Serialize::serialize(&self.0, serializer);

		#[cfg(target_endian = "big")] {
			let mut words = self.0;
			for block in &mut words {
				for word in block {
					*word = u32::from_le(*word);
				}
			}
			serde::Serialize::serialize(&words, serializer)
		}
	}
}

impl<'de> serde::Deserialize<'de> for ChaChaOutput {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		#[cfg(target_endian = "little")]
		return <[[u32; 16]; CN]>::deserialize(deserializer).map(ChaChaOutput);

		#[cfg(target_endian = "big")] {
			let mut words = <[[u32; 16]; CN]>::deserialize(deserializer)?;
			for block in &mut words {
				for word in block {
					*word = word.to_le();
				}
			}
			Ok(ChaChaOutput(words))
		}
	}
}
