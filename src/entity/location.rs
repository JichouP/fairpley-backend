pub mod id;
pub mod r#type;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Location {
    pub id: self::id::LocationId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub r#type: self::r#type::LocationType,
    pub lat: f64,
    pub lng: f64,
}
