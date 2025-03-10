use crate::error::Failure;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{de, Deserialize, Serialize};
use std::{convert::TryFrom, future::Future};

// MARK: Location

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Location {
    pub id: LocationId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub r#type: LocationType,
    pub lat: f64,
    pub lng: f64,
}

impl Location {
    pub fn new(name: String, r#type: LocationType, lat: f64, lng: f64) -> Self {
        Self {
            id: LocationId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            name,
            r#type,
            lat,
            lng,
        }
    }
}

// MARK: LocationId

crate::entity::newtype! {
    #[derive(Debug, Clone, PartialEq,  Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct LocationId(uuid::Uuid);
}

// MARK: LocationType

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
            _ => Err(anyhow::anyhow!("{}: {}", "Invalid location type", value).into()),
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

// MARK: trait

pub trait LocationRepository<C>: Send + Sync {
    fn get_many_locations(
        &self,
        ctx: C,
    ) -> impl Future<Output = Result<Vec<Location>, Failure>> + Send;

    fn get_one_location(
        &self,
        ctx: C,
        id: LocationId,
    ) -> impl Future<Output = Result<Location, Failure>> + Send;

    fn create_one_location(
        &self,
        ctx: C,
        location: Location,
    ) -> impl Future<Output = Result<Location, Failure>> + Send;

    fn update_one_location(
        &self,
        ctx: C,
        id: LocationId,
        location: Location,
    ) -> impl Future<Output = Result<Location, Failure>> + Send;

    fn remove_one_location(
        &self,
        ctx: C,
        id: LocationId,
    ) -> impl Future<Output = Result<Location, Failure>> + Send;
}

impl<T, C> LocationRepository<C> for &T
where
    T: LocationRepository<C>,
{
    fn get_many_locations(
        &self,
        ctx: C,
    ) -> impl Future<Output = Result<Vec<Location>, Failure>> + Send {
        (*self).get_many_locations(ctx)
    }

    fn get_one_location(
        &self,
        ctx: C,
        id: LocationId,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        (*self).get_one_location(ctx, id)
    }

    fn create_one_location(
        &self,
        ctx: C,
        location: Location,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        (*self).create_one_location(ctx, location)
    }

    fn update_one_location(
        &self,
        ctx: C,
        id: LocationId,
        location: Location,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        (*self).update_one_location(ctx, id, location)
    }

    fn remove_one_location(
        &self,
        ctx: C,
        id: LocationId,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        (*self).remove_one_location(ctx, id)
    }
}

pub trait ProvideLocationRepository: Send + Sync {
    type Context<'a>
    where
        Self: 'a;
    type LocationRepository<'a>: LocationRepository<Self::Context<'a>>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_>;
    fn location_repository(&self) -> &Self::LocationRepository<'_>;

    fn get_many_locations(&self) -> impl Future<Output = Result<Vec<Location>, Failure>> + Send {
        let ctx = self.context();
        self.location_repository().get_many_locations(ctx)
    }

    fn get_one_location(
        &self,
        id: LocationId,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        let ctx = self.context();
        self.location_repository().get_one_location(ctx, id)
    }

    fn create_one_location(
        &self,
        location: Location,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        let ctx = self.context();
        self.location_repository()
            .create_one_location(ctx, location)
    }

    fn update_one_location(
        &self,
        id: LocationId,
        location: Location,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        let ctx = self.context();
        self.location_repository()
            .update_one_location(ctx, id, location)
    }

    fn remove_one_location(
        &self,
        id: LocationId,
    ) -> impl Future<Output = Result<Location, Failure>> + Send {
        let ctx = self.context();
        self.location_repository().remove_one_location(ctx, id)
    }
}

impl<T> ProvideLocationRepository for &T
where
    T: ProvideLocationRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type LocationRepository<'a>
        = T::LocationRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        (*self).context()
    }

    fn location_repository(&self) -> &Self::LocationRepository<'_> {
        (*self).location_repository()
    }
}

impl<T> ProvideLocationRepository for std::sync::Arc<T>
where
    T: ProvideLocationRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type LocationRepository<'a>
        = T::LocationRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        <T as ProvideLocationRepository>::context(self)
    }

    fn location_repository(&self) -> &Self::LocationRepository<'_> {
        <T as ProvideLocationRepository>::location_repository(self)
    }
}

// MARK: tests

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
    fn test_serialize_location_id() {
        let uuid = Uuid::now_v7();
        let location_id = LocationId::new(uuid);

        let serialized = serde_json::to_string(&location_id).unwrap();
        assert_eq!(serialized, format!("\"{}\"", uuid));
    }

    #[rstest]
    fn test_deserialize_location_id() {
        let uuid = Uuid::now_v7();
        let json = format!("\"{}\"", uuid);

        let deserialized: LocationId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.into_inner(), uuid);
    }

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
        assert!(err.to_string().contains("Invalid location type"));
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
    fn test_serialize_location_type(#[case] location_type: LocationType, #[case] expected: &str) {
        let serialized = serde_json::to_string(&location_type).unwrap();
        assert_eq!(serialized, expected);
    }

    #[rstest]
    #[case(LocationType::Campsite, "campsite")]
    #[case(LocationType::Home, "home")]
    #[case(LocationType::Other, "other")]
    #[case(LocationType::Store, "store")]
    fn test_display_location_type(#[case] location_type: LocationType, #[case] expected: &str) {
        assert_eq!(location_type.to_string(), expected);
    }
}
