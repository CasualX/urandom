use super::*;

impl serde::Serialize for Dice {
	#[inline]
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_newtype_struct("Dice", &self.0.range)
	}
}

impl<'de> serde::Deserialize<'de> for Dice {
	#[inline]
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct DiceVisitor;

		impl<'de> serde::de::Visitor<'de> for DiceVisitor {
			type Value = Dice;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a Dice containing a non-zero number of sides")
			}

			fn visit_newtype_struct<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Dice, D::Error> {
				let sides = <u8 as serde::Deserialize>::deserialize(deserializer)?;
				if sides == 0 {
					return Err(serde::de::Error::custom("a dice must have at least one side"));
				}
				Ok(Dice::new(sides))
			}
		}

		deserializer.deserialize_newtype_struct("Dice", DiceVisitor)
	}
}

#[test]
fn serde_encodes_number_of_sides() {
	assert_eq!(serde_json::to_string(&Dice::D20).unwrap(), "20");

	let dice: Dice = serde_json::from_str("6").unwrap();
	assert_eq!(serde_json::to_string(&dice).unwrap(), "6");
}

#[test]
fn serde_rejects_zero_sides() {
	assert!(serde_json::from_str::<Dice>("0").is_err());
}
