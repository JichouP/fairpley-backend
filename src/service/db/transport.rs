pub type TransportRecord = crate::entity::transport::Transport;

pub struct SelectOneTransportResponse(TransportRecord);

impl From<TransportRecord> for SelectOneTransportResponse {
    fn from(value: TransportRecord) -> Self {
        Self(value)
    }
}

impl SelectOneTransportResponse {
    pub fn as_inner(&self) -> &TransportRecord {
        &self.0
    }
}

pub type SelectManyTransportResponseItem = TransportRecord;

pub struct SelectManyTransportsResponse(Vec<SelectManyTransportResponseItem>);

impl From<Vec<TransportRecord>> for SelectManyTransportsResponse {
    fn from(values: Vec<TransportRecord>) -> Self {
        Self(values)
    }
}

impl SelectManyTransportsResponse {
    pub fn as_inner(&self) -> &Vec<SelectManyTransportResponseItem> {
        &self.0
    }
}

pub struct InsertOneTransportRequest {
    pub id: crate::entity::transport::id::TransportId,
    pub event_id: crate::entity::event::id::EventId,
    pub r#type: String,
    pub capacity: i32,
    pub departure_location_id: crate::entity::location::id::LocationId,
    pub arrival_location_id: crate::entity::location::id::LocationId,
    pub departure_at: chrono::DateTime<chrono::Utc>,
    pub arrival_at: chrono::DateTime<chrono::Utc>,
}

pub struct InsertOneTransportResponse(TransportRecord);

impl From<TransportRecord> for InsertOneTransportResponse {
    fn from(value: TransportRecord) -> Self {
        Self(value)
    }
}

impl InsertOneTransportResponse {
    pub fn as_inner(&self) -> &TransportRecord {
        &self.0
    }
}

pub struct UpdateOneTransportRequest {
    pub r#type: String,
    pub capacity: i32,
    pub departure_location_id: crate::entity::location::id::LocationId,
    pub arrival_location_id: crate::entity::location::id::LocationId,
    pub departure_at: chrono::DateTime<chrono::Utc>,
    pub arrival_at: chrono::DateTime<chrono::Utc>,
}

pub struct UpdateOneTransportResponse(TransportRecord);

impl From<TransportRecord> for UpdateOneTransportResponse {
    fn from(value: TransportRecord) -> Self {
        Self(value)
    }
}

impl UpdateOneTransportResponse {
    pub fn as_inner(&self) -> &TransportRecord {
        &self.0
    }
}

pub struct DeleteOneTransportResponse(TransportRecord);

impl From<TransportRecord> for DeleteOneTransportResponse {
    fn from(value: TransportRecord) -> Self {
        Self(value)
    }
}

impl DeleteOneTransportResponse {
    pub fn as_inner(&self) -> &TransportRecord {
        &self.0
    }
}

pub trait DbTransportAdapter: Clone + Send + Sync + 'static {
    fn select_one_transport_by_id(
        &self,
        id: crate::entity::transport::id::TransportId,
    ) -> impl ::std::future::Future<Output = Result<SelectOneTransportResponse, crate::error::Failure>>
           + Send;

    fn select_many_transports(
        &self,
    ) -> impl ::std::future::Future<
        Output = Result<SelectManyTransportsResponse, crate::error::Failure>,
    > + Send;

    fn insert_one_transport(
        &self,
        transport: InsertOneTransportRequest,
    ) -> impl ::std::future::Future<Output = Result<InsertOneTransportResponse, crate::error::Failure>>
           + Send;

    fn update_one_transport_by_id(
        &self,
        id: crate::entity::transport::id::TransportId,
        transport: UpdateOneTransportRequest,
    ) -> impl ::std::future::Future<Output = Result<UpdateOneTransportResponse, crate::error::Failure>>
           + Send;

    fn delete_one_transport_by_id(
        &self,
        id: crate::entity::transport::id::TransportId,
    ) -> impl ::std::future::Future<Output = Result<DeleteOneTransportResponse, crate::error::Failure>>
           + Send;
}
