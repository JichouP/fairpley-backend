use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod id;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Purchase {
    pub id: self::id::PurchaseId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user: super::user::User,
    pub event: super::event::Event,
    pub name: String,
    pub price: i32,
}

impl Purchase {
    pub fn new(
        user: super::user::User,
        event: super::event::Event,
        name: String,
        price: i32,
    ) -> Self {
        Self {
            id: id::PurchaseId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            user,
            event,
            name,
            price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::event::Event;
    use crate::entity::user::User;

    #[test]
    fn test_purchase_new() {
        // テスト用のユーザーとイベントを作成
        let user = User::new("test_user".to_string());
        let event = Event::new(
            "test_event".to_string(),
            chrono::Utc::now(),
            chrono::Utc::now(),
        );

        // Purchaseインスタンスを作成
        let purchase = Purchase::new(user.clone(), event.clone(), "test_name".to_string(), 1000);

        // 各フィールドの検証
        assert!(!purchase.id.as_inner().to_string().is_empty());
        assert!(purchase.created_at <= chrono::Utc::now());
        assert!(purchase.updated_at <= chrono::Utc::now());
        assert_eq!(purchase.user, user);
        assert_eq!(purchase.event, event);
        assert_eq!(purchase.name, "test_name");
        assert_eq!(purchase.price, 1000);
    }
}
