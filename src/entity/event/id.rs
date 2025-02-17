use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

crate::entity::newtype! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct EventId(uuid::Uuid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;

    #[test]
    fn test_event_id() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::new(uuid);

        assert_eq!(uuid, event_id.into_inner());
    }

    #[rstest]
    fn test_serialize() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::new(uuid);

        let serialized = serde_json::to_string(&event_id).unwrap();
        assert_eq!(serialized, format!("\"{}\"", uuid));
    }

    #[rstest]
    fn test_deserialize() {
        let uuid = Uuid::now_v7();
        let json = format!("\"{}\"", uuid);

        let deserialized: EventId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.into_inner(), uuid);
    }
}
