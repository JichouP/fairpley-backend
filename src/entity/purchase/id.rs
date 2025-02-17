use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

crate::entity::newtype! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct PurchaseId(uuid::Uuid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_purchase_id() {
        let uuid = Uuid::now_v7();
        let purchase_id = PurchaseId::new(uuid);

        assert_eq!(uuid, purchase_id.into_inner());
    }
}
