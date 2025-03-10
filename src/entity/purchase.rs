use crate::error::Failure;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

// MARK: Purchase

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Purchase {
    pub id: PurchaseId,
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
            id: PurchaseId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            user,
            event,
            name,
            price,
        }
    }
}

// MARK: PurchaseId

crate::entity::newtype! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct PurchaseId(uuid::Uuid);
}

// MARK: trait

pub trait PurchaseRepository<C>: Send + Sync {
    fn get_many_purchases(
        &self,
        ctx: C,
    ) -> impl Future<Output = Result<Vec<Purchase>, Failure>> + Send;

    fn get_one_purchase(
        &self,
        ctx: C,
        id: PurchaseId,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send;

    fn create_one_purchase(
        &self,
        ctx: C,
        purchase: Purchase,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send;

    fn update_one_purchase(
        &self,
        ctx: C,
        id: PurchaseId,
        purchase: Purchase,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send;

    fn remove_one_purchase(
        &self,
        ctx: C,
        id: PurchaseId,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send;
}

impl<T, C> PurchaseRepository<C> for &T
where
    T: PurchaseRepository<C>,
{
    fn get_many_purchases(
        &self,
        ctx: C,
    ) -> impl Future<Output = Result<Vec<Purchase>, Failure>> + Send {
        (*self).get_many_purchases(ctx)
    }

    fn get_one_purchase(
        &self,
        ctx: C,
        id: PurchaseId,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        (*self).get_one_purchase(ctx, id)
    }

    fn create_one_purchase(
        &self,
        ctx: C,
        purchase: Purchase,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        (*self).create_one_purchase(ctx, purchase)
    }

    fn update_one_purchase(
        &self,
        ctx: C,
        id: PurchaseId,
        purchase: Purchase,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        (*self).update_one_purchase(ctx, id, purchase)
    }

    fn remove_one_purchase(
        &self,
        ctx: C,
        id: PurchaseId,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        (*self).remove_one_purchase(ctx, id)
    }
}

pub trait ProvidePurchaseRepository: Send + Sync {
    type Context<'a>
    where
        Self: 'a;
    type PurchaseRepository<'a>: PurchaseRepository<Self::Context<'a>>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_>;
    fn purchase_repository(&self) -> &Self::PurchaseRepository<'_>;

    fn get_many_purchases(&self) -> impl Future<Output = Result<Vec<Purchase>, Failure>> + Send {
        let ctx = self.context();
        self.purchase_repository().get_many_purchases(ctx)
    }

    fn get_one_purchase(
        &self,
        id: PurchaseId,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        let ctx = self.context();
        self.purchase_repository().get_one_purchase(ctx, id)
    }

    fn create_one_purchase(
        &self,
        purchase: Purchase,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        let ctx = self.context();
        self.purchase_repository()
            .create_one_purchase(ctx, purchase)
    }

    fn update_one_purchase(
        &self,
        id: PurchaseId,
        purchase: Purchase,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        let ctx = self.context();
        self.purchase_repository()
            .update_one_purchase(ctx, id, purchase)
    }

    fn remove_one_purchase(
        &self,
        id: PurchaseId,
    ) -> impl Future<Output = Result<Purchase, Failure>> + Send {
        let ctx = self.context();
        self.purchase_repository().remove_one_purchase(ctx, id)
    }
}

impl<T> ProvidePurchaseRepository for &T
where
    T: ProvidePurchaseRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type PurchaseRepository<'a>
        = T::PurchaseRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        (*self).context()
    }

    fn purchase_repository(&self) -> &Self::PurchaseRepository<'_> {
        (*self).purchase_repository()
    }
}

impl<T> ProvidePurchaseRepository for std::sync::Arc<T>
where
    T: ProvidePurchaseRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type PurchaseRepository<'a>
        = T::PurchaseRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        <T as ProvidePurchaseRepository>::context(self)
    }

    fn purchase_repository(&self) -> &Self::PurchaseRepository<'_> {
        <T as ProvidePurchaseRepository>::purchase_repository(self)
    }
}

// MARK: tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::event::Event;
    use crate::entity::user::User;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
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

    #[rstest]
    fn test_serialize_purchase_id() {
        let uuid = Uuid::now_v7();
        let purchase_id = PurchaseId::new(uuid);

        let serialized = serde_json::to_string(&purchase_id).unwrap();
        assert_eq!(serialized, format!("\"{}\"", uuid));
    }

    #[rstest]
    fn test_deserialize_purchase_id() {
        let uuid = Uuid::now_v7();
        let json = format!("\"{}\"", uuid);

        let deserialized: PurchaseId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.into_inner(), uuid);
    }
}
