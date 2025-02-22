pub type EventRecord = crate::entity::event::Event;

pub struct SelectOneEventResponse(EventRecord);

impl From<EventRecord> for SelectOneEventResponse {
    fn from(value: EventRecord) -> Self {
        Self(value)
    }
}

impl SelectOneEventResponse {
    pub fn as_inner(&self) -> &EventRecord {
        &self.0
    }
}

pub type SelectManyEventResponseItem = EventRecord;

pub struct SelectManyEventsResponse(Vec<SelectManyEventResponseItem>);

impl From<Vec<EventRecord>> for SelectManyEventsResponse {
    fn from(values: Vec<EventRecord>) -> Self {
        Self(values)
    }
}

impl SelectManyEventsResponse {
    pub fn as_inner(&self) -> &Vec<SelectManyEventResponseItem> {
        &self.0
    }
}

pub struct InsertOneEventRequest {
    pub id: crate::entity::event::id::EventId,
    pub name: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

pub struct InsertOneEventResponse(EventRecord);

impl From<EventRecord> for InsertOneEventResponse {
    fn from(value: EventRecord) -> Self {
        Self(value)
    }
}

impl InsertOneEventResponse {
    pub fn as_inner(&self) -> &EventRecord {
        &self.0
    }
}

pub struct UpdateOneEventRequest {
    pub name: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

pub struct UpdateOneEventResponse(EventRecord);

impl From<EventRecord> for UpdateOneEventResponse {
    fn from(value: EventRecord) -> Self {
        Self(value)
    }
}

impl UpdateOneEventResponse {
    pub fn as_inner(&self) -> &EventRecord {
        &self.0
    }
}

pub struct DeleteOneEventResponse(EventRecord);

impl From<EventRecord> for DeleteOneEventResponse {
    fn from(value: EventRecord) -> Self {
        Self(value)
    }
}

impl DeleteOneEventResponse {
    pub fn as_inner(&self) -> &EventRecord {
        &self.0
    }
}

pub trait DbEventAdapter: Clone + Send + Sync + 'static {
    fn select_one_event_by_id(
        &self,
        id: crate::entity::event::id::EventId,
    ) -> impl ::std::future::Future<Output = Result<SelectOneEventResponse, crate::error::Failure>> + Send;

    fn select_many_events(
        &self,
    ) -> impl ::std::future::Future<Output = Result<SelectManyEventsResponse, crate::error::Failure>>
           + Send;

    fn insert_one_event(
        &self,
        event: InsertOneEventRequest,
    ) -> impl ::std::future::Future<Output = Result<InsertOneEventResponse, crate::error::Failure>> + Send;

    fn update_one_event_by_id(
        &self,
        id: crate::entity::event::id::EventId,
        event: UpdateOneEventRequest,
    ) -> impl ::std::future::Future<Output = Result<UpdateOneEventResponse, crate::error::Failure>> + Send;

    fn delete_one_event_by_id(
        &self,
        id: crate::entity::event::id::EventId,
    ) -> impl ::std::future::Future<Output = Result<DeleteOneEventResponse, crate::error::Failure>> + Send;
}
