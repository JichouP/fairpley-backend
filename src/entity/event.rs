use crate::error::Failure;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;

// MARK: Event

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Event {
    pub id: EventId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

impl Event {
    pub fn new(
        name: String,
        started_at: chrono::DateTime<chrono::Utc>,
        ended_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id: EventId::new(uuid::Uuid::now_v7()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            name,
            started_at,
            ended_at,
        }
    }
}

// MARK: EventId

crate::entity::newtype! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct EventId(uuid::Uuid);
}

// MARK: EventRepository

pub trait EventRepository<C>: Send + Sync {
    fn get_many_events(&self, ctx: C) -> impl Future<Output = Result<Vec<Event>, Failure>> + Send;

    fn get_one_event(
        &self,
        ctx: C,
        id: EventId,
    ) -> impl Future<Output = Result<Event, Failure>> + Send;

    fn create_one_event(
        &self,
        ctx: C,
        event: Event,
    ) -> impl Future<Output = Result<Event, Failure>> + Send;

    fn update_one_event(
        &self,
        ctx: C,
        id: EventId,
        event: Event,
    ) -> impl Future<Output = Result<Event, Failure>> + Send;

    fn remove_one_event(
        &self,
        ctx: C,
        id: EventId,
    ) -> impl Future<Output = Result<Event, Failure>> + Send;
}

impl<T, C> EventRepository<C> for &T
where
    T: EventRepository<C>,
{
    fn get_many_events(&self, ctx: C) -> impl Future<Output = Result<Vec<Event>, Failure>> + Send {
        (*self).get_many_events(ctx)
    }

    fn get_one_event(
        &self,
        ctx: C,
        id: EventId,
    ) -> impl Future<Output = Result<Event, Failure>> + Send {
        (*self).get_one_event(ctx, id)
    }

    fn create_one_event(
        &self,
        ctx: C,
        event: Event,
    ) -> impl Future<Output = Result<Event, Failure>> + Send {
        (*self).create_one_event(ctx, event)
    }

    fn update_one_event(
        &self,
        ctx: C,
        id: EventId,
        event: Event,
    ) -> impl Future<Output = Result<Event, Failure>> + Send {
        (*self).update_one_event(ctx, id, event)
    }

    fn remove_one_event(
        &self,
        ctx: C,
        id: EventId,
    ) -> impl Future<Output = Result<Event, Failure>> + Send {
        (*self).remove_one_event(ctx, id)
    }
}

pub trait ProvideEventRepository: Send + Sync {
    type Context<'a>
    where
        Self: 'a;
    type EventRepository<'a>: EventRepository<Self::Context<'a>>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_>;
    fn event_repository(&self) -> &Self::EventRepository<'_>;

    fn get_many_events(&self) -> impl Future<Output = Result<Vec<Event>, Failure>> + Send {
        let ctx = self.context();
        self.event_repository().get_many_events(ctx)
    }

    fn get_one_event(&self, id: EventId) -> impl Future<Output = Result<Event, Failure>> + Send {
        let ctx = self.context();
        self.event_repository().get_one_event(ctx, id)
    }

    fn create_one_event(
        &self,
        event: Event,
    ) -> impl Future<Output = Result<Event, Failure>> + Send {
        let ctx = self.context();
        self.event_repository().create_one_event(ctx, event)
    }

    fn update_one_event(
        &self,
        id: EventId,
        event: Event,
    ) -> impl Future<Output = Result<Event, Failure>> + Send {
        let ctx = self.context();
        self.event_repository().update_one_event(ctx, id, event)
    }

    fn remove_one_event(&self, id: EventId) -> impl Future<Output = Result<Event, Failure>> + Send {
        let ctx = self.context();
        self.event_repository().remove_one_event(ctx, id)
    }
}

impl<T> ProvideEventRepository for &T
where
    T: ProvideEventRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type EventRepository<'a>
        = T::EventRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        (*self).context()
    }

    fn event_repository(&self) -> &Self::EventRepository<'_> {
        (*self).event_repository()
    }
}

impl<T> ProvideEventRepository for std::sync::Arc<T>
where
    T: ProvideEventRepository,
{
    type Context<'a>
        = T::Context<'a>
    where
        Self: 'a;
    type EventRepository<'a>
        = T::EventRepository<'a>
    where
        Self: 'a;

    fn context(&self) -> Self::Context<'_> {
        <T as ProvideEventRepository>::context(self)
    }

    fn event_repository(&self) -> &Self::EventRepository<'_> {
        <T as ProvideEventRepository>::event_repository(self)
    }
}

// MARK: tests

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
    fn test_event_new() {
        let started_at = chrono::Utc::now();
        let ended_at = chrono::Utc::now();
        let event = Event::new("test_name".to_string(), started_at, ended_at);

        assert_eq!(event.name, "test_name");
        assert_eq!(event.started_at, started_at);
        assert_eq!(event.ended_at, ended_at);
    }

    #[rstest]
    fn test_serialize_event_id() {
        let uuid = Uuid::now_v7();
        let event_id = EventId::new(uuid);

        let serialized = serde_json::to_string(&event_id).unwrap();
        assert_eq!(serialized, format!("\"{}\"", uuid));
    }

    #[rstest]
    fn test_deserialize_event_id() {
        let uuid = Uuid::now_v7();
        let json = format!("\"{}\"", uuid);

        let deserialized: EventId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.into_inner(), uuid);
    }
}
