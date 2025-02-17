use schemars::JsonSchema;
use serde::{de, Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    Campsite,
    Home,
    Other,
    Store,
}

impl TryFrom<&str> for LocationType {
    type Error = crate::error::Failure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "campsite" => Ok(LocationType::Campsite),
            "home" => Ok(LocationType::Home),
            "other" => Ok(LocationType::Other),
            "store" => Ok(LocationType::Store),
            _ => Err(anyhow::anyhow!(
                "{}: {}",
                crate::error::messages::INVALID_LOCATION_TYPE,
                value
            )
            .into()),
        }
    }
}

impl AsRef<str> for LocationType {
    fn as_ref(&self) -> &str {
        match self {
            LocationType::Campsite => "campsite",
            LocationType::Home => "home",
            LocationType::Other => "other",
            LocationType::Store => "store",
        }
    }
}

impl Serialize for LocationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for LocationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let type_str = String::deserialize(deserializer)?;
        LocationType::try_from(type_str.as_str()).map_err(|e| de::Error::custom(e.to_string()))
    }
}

impl ::core::fmt::Display for LocationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("campsite", LocationType::Campsite)]
    #[case("home", LocationType::Home)]
    #[case("other", LocationType::Other)]
    #[case("store", LocationType::Store)]
    fn test_try_from_str_valid(#[case] input: &str, #[case] expected: LocationType) {
        let result = LocationType::try_from(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("\"campsite\"", LocationType::Campsite)]
    #[case("\"home\"", LocationType::Home)]
    #[case("\"other\"", LocationType::Other)]
    #[case("\"store\"", LocationType::Store)]
    fn test_deserialize_valid(#[case] json: &str, #[case] expected: LocationType) {
        let deserialized: LocationType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, expected);
    }

    #[rstest]
    #[case("")]
    #[case("invalid")]
    #[case("CAMPSITE")]
    #[case("home ")]
    #[case(" store")]
    fn test_try_from_str_invalid(#[case] input: &str) {
        let result = LocationType::try_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .to_string()
            .contains(crate::error::messages::INVALID_LOCATION_TYPE));
    }

    #[rstest]
    #[case("\"\"")]
    #[case("\"invalid\"")]
    #[case("\"STORE\"")]
    #[case("null")]
    #[case("123")]
    #[case("{}")]
    fn test_deserialize_invalid(#[case] invalid_json: &str) {
        let result = serde_json::from_str::<LocationType>(invalid_json);
        assert!(result.is_err());
    }

    #[rstest]
    #[case(LocationType::Campsite, "\"campsite\"")]
    #[case(LocationType::Home, "\"home\"")]
    #[case(LocationType::Other, "\"other\"")]
    #[case(LocationType::Store, "\"store\"")]
    fn test_serialize(#[case] location_type: LocationType, #[case] expected: &str) {
        let serialized = serde_json::to_string(&location_type).unwrap();
        assert_eq!(serialized, expected);
    }

    #[rstest]
    #[case(LocationType::Campsite, "campsite")]
    #[case(LocationType::Home, "home")]
    #[case(LocationType::Other, "other")]
    #[case(LocationType::Store, "store")]
    fn test_display(#[case] location_type: LocationType, #[case] expected: &str) {
        assert_eq!(location_type.to_string(), expected);
    }
}
