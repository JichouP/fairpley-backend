use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

crate::entity::newtype! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct TransportId(uuid::Uuid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_transport_id() {
        let uuid = Uuid::now_v7();
        let transport_id = TransportId::new(uuid);

        assert_eq!(uuid, transport_id.into_inner());
    }
}
