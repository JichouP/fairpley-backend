use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

crate::entity::newtype! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct EventId(uuid::Uuid);
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_event_id() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::new(uuid);

        assert_eq!(uuid, event_id.into_inner());
    }
}
