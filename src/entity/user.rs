use crate::error::Failure;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

// MARK: User

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: UserId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: UserId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            name,
        }
    }
}

// MARK: UserId

crate::entity::newtype! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct UserId(uuid::Uuid);
}

// MARK: trait

pub trait UserRepository<C>: Send + Sync {
    fn get_many_users(&self, ctx: C) -> impl Future<Output = Result<Vec<User>, Failure>> + Send;

    fn get_one_user(
        &self,
        ctx: C,
        id: UserId,
    ) -> impl Future<Output = Result<User, Failure>> + Send;

    fn create_one_user(
        &self,
        ctx: C,
        user: User,
    ) -> impl Future<Output = Result<User, Failure>> + Send;

    fn update_one_user(
        &self,
        ctx: C,
        id: UserId,
        user: User,
    ) -> impl Future<Output = Result<User, Failure>> + Send;

    fn remove_one_user(
        &self,
        ctx: C,
        id: UserId,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
}

impl<U, C> UserRepository<C> for &U
where
    U: UserRepository<C>,
{
    fn get_many_users(&self, ctx: C) -> impl Future<Output = Result<Vec<User>, Failure>> + Send {
        (*self).get_many_users(ctx)
    }

    fn get_one_user(
        &self,
        ctx: C,
        id: UserId,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        (*self).get_one_user(ctx, id)
    }

    fn create_one_user(
        &self,
        ctx: C,
        user: User,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        (*self).create_one_user(ctx, user)
    }

    fn update_one_user(
        &self,
        ctx: C,
        id: UserId,
        user: User,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        (*self).update_one_user(ctx, id, user)
    }

    fn remove_one_user(
        &self,
        ctx: C,
        id: UserId,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        (*self).remove_one_user(ctx, id)
    }
}

pub trait ProvideUserRepository: Send + Sync {
    type Context<'a>
    where
        Self: 'a;
    type UserRepository<'a>: UserRepository<Self::Context<'a>>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_>;
    fn user_repository(&self) -> &Self::UserRepository<'_>;

    fn get_many_users(&self) -> impl Future<Output = Result<Vec<User>, Failure>> + Send {
        let ctx = self.context();
        self.user_repository().get_many_users(ctx)
    }
    fn get_one_user(&self, id: UserId) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_repository().get_one_user(ctx, id)
    }
    fn create_one_user(&self, user: User) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_repository().create_one_user(ctx, user)
    }
    fn update_one_user(
        &self,
        id: UserId,
        user: User,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_repository().update_one_user(ctx, id, user)
    }
    fn remove_one_user(&self, id: UserId) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_repository().remove_one_user(ctx, id)
    }
}

impl<T> ProvideUserRepository for &T
where
    T: ProvideUserRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type UserRepository<'a>
        = T::UserRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        (*self).context()
    }
    fn user_repository(&self) -> &Self::UserRepository<'_> {
        (*self).user_repository()
    }
}

impl<T> ProvideUserRepository for std::sync::Arc<T>
where
    T: ProvideUserRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type UserRepository<'a>
        = T::UserRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        <T as ProvideUserRepository>::context(self)
    }
    fn user_repository(&self) -> &Self::UserRepository<'_> {
        <T as ProvideUserRepository>::user_repository(self)
    }
}
// MARK: tests

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
    fn test_user_new() {
        let user = User::new("test_name".to_string());
        assert_eq!(user.name, "test_name");
    }

    #[rstest]
    fn test_serialize_user_id() {
        let uuid = Uuid::now_v7();
        let user_id = UserId::new(uuid);

        let serialized = serde_json::to_string(&user_id).unwrap();
        assert_eq!(serialized, format!("\"{}\"", uuid));
    }

    #[rstest]
    fn test_deserialize_user_id() {
        let uuid = Uuid::now_v7();
        let json = format!("\"{}\"", uuid);

        let deserialized: UserId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.into_inner(), uuid);
    }
}
