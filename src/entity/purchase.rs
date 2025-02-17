use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod id;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Purchase {
    pub id: self::id::PurchaseId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user: super::user::User,
    pub event: super::event::Event,
}
