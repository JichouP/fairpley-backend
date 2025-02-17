use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

crate::entity::newtype! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct UserId(uuid::Uuid);
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_user_id() {
        let uuid = Uuid::now_v7();
        let user_id = UserId::new(uuid);

        assert_eq!(uuid, user_id.into_inner());
    }
}
