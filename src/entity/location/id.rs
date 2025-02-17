use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

crate::entity::newtype! {
    #[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct LocationId(uuid::Uuid);
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_location_id() {
        let uuid = Uuid::now_v7();
        let location_id = LocationId::new(uuid);

        assert_eq!(uuid, location_id.into_inner());
    }
}
