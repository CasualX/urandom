use super::*;

#[derive(serde::Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
	State,
	Index,
	Random,
	#[serde(other)]
	Other,
}

impl<T> serde::Serialize for BlockRngImpl<T>
where
	T: BlockRng + serde::Serialize,
	T::Output: serde::Serialize,
{
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeMap;

		let has_cache = self.index < mem::size_of::<T::Output>() as u32;
		let mut map = serializer.serialize_map(Some(if has_cache { 3 } else { 1 }))?;
		map.serialize_entry("state", &self.state)?;
		if has_cache {
			map.serialize_entry("index", &self.index)?;
			map.serialize_entry("random", &self.random)?;
		}
		map.end()
	}
}

impl<'de, T> serde::Deserialize<'de> for BlockRngImpl<T>
where
	T: BlockRng + serde::Deserialize<'de>,
	T::Output: serde::Deserialize<'de>,
{
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct BlockRngVisitor<T: BlockRng>(core::marker::PhantomData<T>);

		impl<'de, T> serde::de::Visitor<'de> for BlockRngVisitor<T>
		where
			T: BlockRng + serde::Deserialize<'de>,
			T::Output: serde::Deserialize<'de>,
		{
			type Value = BlockRngImpl<T>;

			fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
				formatter.write_str("a random block generator state")
			}

			fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
				let mut state = None;
				let mut index = None;
				let mut random = None;

				while let Some(field) = map.next_key()? {
					match field {
						Field::State => {
							if state.is_some() {
								return Err(serde::de::Error::duplicate_field("state"));
							}
							state = Some(map.next_value()?);
						}
						Field::Index => {
							if index.is_some() {
								return Err(serde::de::Error::duplicate_field("index"));
							}
							index = Some(map.next_value()?);
						}
						Field::Random => {
							if random.is_some() {
								return Err(serde::de::Error::duplicate_field("random"));
							}
							random = Some(map.next_value()?);
						}
						Field::Other => {
							let _ = map.next_value::<serde::de::IgnoredAny>()?;
						}
					}
				}

				let state = state.ok_or_else(|| serde::de::Error::missing_field("state"))?;
				let (index, random) = match (index, random) {
					(Some(index), Some(random)) => (index, random),
					(Some(_), None) => return Err(serde::de::Error::missing_field("random")),
					(None, Some(_)) => return Err(serde::de::Error::missing_field("index")),
					(None, None) => (!0, T::Output::default()),
				};

				Ok(BlockRngImpl { state, index, random })
			}
		}

		deserializer.deserialize_map(BlockRngVisitor(core::marker::PhantomData))
	}
}
