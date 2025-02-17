use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod id;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: self::id::UserId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: id::UserId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            name,
        }
    }
}
