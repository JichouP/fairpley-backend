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
            _ => Err(crate::error::Failure::reject_bad_request(format!(
                "無効なロケーションタイプです: {}",
                value
            ))),
        }
    }
}

impl From<&LocationType> for String {
    fn from(value: &LocationType) -> Self {
        match value {
            LocationType::Campsite => "campsite".to_string(),
            LocationType::Home => "home".to_string(),
            LocationType::Other => "other".to_string(),
            LocationType::Store => "store".to_string(),
        }
    }
}

impl Serialize for LocationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let r#type: String = self.into();
        serializer.serialize_str(&r#type)
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
        let r#type: String = self.into();
        write!(f, "{}", r#type)
    }
}
