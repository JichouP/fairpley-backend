use crate::error::Failure;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

// MARK: Transport

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Transport {
    pub id: TransportId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub r#type: String,
}

impl Transport {
    pub fn new(name: String, r#type: String) -> Self {
        Self {
            id: TransportId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            name,
            r#type,
        }
    }
}

// MARK: TransportId

crate::entity::newtype! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct TransportId(uuid::Uuid);
}

// MARK: trait

pub trait TransportRepository<C>: Send + Sync {
    fn get_many_transports(
        &self,
        ctx: C,
    ) -> impl Future<Output = Result<Vec<Transport>, Failure>> + Send;

    fn get_one_transport(
        &self,
        ctx: C,
        id: TransportId,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send;

    fn create_one_transport(
        &self,
        ctx: C,
        transport: Transport,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send;

    fn update_one_transport(
        &self,
        ctx: C,
        id: TransportId,
        transport: Transport,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send;

    fn remove_one_transport(
        &self,
        ctx: C,
        id: TransportId,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send;
}

impl<T, C> TransportRepository<C> for &T
where
    T: TransportRepository<C>,
{
    fn get_many_transports(
        &self,
        ctx: C,
    ) -> impl Future<Output = Result<Vec<Transport>, Failure>> + Send {
        (*self).get_many_transports(ctx)
    }

    fn get_one_transport(
        &self,
        ctx: C,
        id: TransportId,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        (*self).get_one_transport(ctx, id)
    }

    fn create_one_transport(
        &self,
        ctx: C,
        transport: Transport,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        (*self).create_one_transport(ctx, transport)
    }

    fn update_one_transport(
        &self,
        ctx: C,
        id: TransportId,
        transport: Transport,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        (*self).update_one_transport(ctx, id, transport)
    }

    fn remove_one_transport(
        &self,
        ctx: C,
        id: TransportId,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        (*self).remove_one_transport(ctx, id)
    }
}

pub trait ProvideTransportRepository: Send + Sync {
    type Context<'a>
    where
        Self: 'a;
    type TransportRepository<'a>: TransportRepository<Self::Context<'a>>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_>;
    fn transport_repository(&self) -> &Self::TransportRepository<'_>;

    fn get_many_transports(&self) -> impl Future<Output = Result<Vec<Transport>, Failure>> + Send {
        let ctx = self.context();
        self.transport_repository().get_many_transports(ctx)
    }

    fn get_one_transport(
        &self,
        id: TransportId,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        let ctx = self.context();
        self.transport_repository().get_one_transport(ctx, id)
    }

    fn create_one_transport(
        &self,
        transport: Transport,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        let ctx = self.context();
        self.transport_repository()
            .create_one_transport(ctx, transport)
    }

    fn update_one_transport(
        &self,
        id: TransportId,
        transport: Transport,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        let ctx = self.context();
        self.transport_repository()
            .update_one_transport(ctx, id, transport)
    }

    fn remove_one_transport(
        &self,
        id: TransportId,
    ) -> impl Future<Output = Result<Transport, Failure>> + Send {
        let ctx = self.context();
        self.transport_repository().remove_one_transport(ctx, id)
    }
}

impl<T> ProvideTransportRepository for &T
where
    T: ProvideTransportRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type TransportRepository<'a>
        = T::TransportRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        (*self).context()
    }
    fn transport_repository(&self) -> &Self::TransportRepository<'_> {
        (*self).transport_repository()
    }
}

impl<T> ProvideTransportRepository for std::sync::Arc<T>
where
    T: ProvideTransportRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type TransportRepository<'a>
        = T::TransportRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        <T as ProvideTransportRepository>::context(self)
    }
    fn transport_repository(&self) -> &Self::TransportRepository<'_> {
        <T as ProvideTransportRepository>::transport_repository(self)
    }
}

// MARK: tests

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
    fn test_transport_new() {
        let transport = Transport::new("test_name".to_string(), "test_type".to_string());
        assert_eq!(transport.name, "test_name");
        assert_eq!(transport.r#type, "test_type");
    }

    #[rstest]
    fn test_serialize_transport_id() {
        let uuid = Uuid::now_v7();
        let transport_id = TransportId::new(uuid);

        let serialized = serde_json::to_string(&transport_id).unwrap();
        assert_eq!(serialized, format!("\"{}\"", uuid));
    }

    #[rstest]
    fn test_deserialize_transport_id() {
        let uuid = Uuid::now_v7();
        let json = format!("\"{}\"", uuid);

        let deserialized: TransportId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.into_inner(), uuid);
    }
}
