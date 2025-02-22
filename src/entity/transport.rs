pub mod id;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Transport {
    pub id: self::id::TransportId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub r#type: String,
}

impl Transport {
    pub fn new(name: String, r#type: String) -> Self {
        Self {
            id: id::TransportId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            name,
            r#type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_new() {
        let transport = Transport::new("test_name".to_string(), "test_type".to_string());
        assert_eq!(transport.name, "test_name");
        assert_eq!(transport.r#type, "test_type");
    }
}
