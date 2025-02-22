pub type LocationRecord = crate::entity::location::Location;

pub struct SelectOneLocationResponse(LocationRecord);

impl From<LocationRecord> for SelectOneLocationResponse {
    fn from(value: LocationRecord) -> Self {
        Self(value)
    }
}

impl SelectOneLocationResponse {
    pub fn as_inner(&self) -> &LocationRecord {
        &self.0
    }
}

pub type SelectManyLocationResponseItem = LocationRecord;

pub struct SelectManyLocationsResponse(Vec<SelectManyLocationResponseItem>);

impl From<Vec<LocationRecord>> for SelectManyLocationsResponse {
    fn from(values: Vec<LocationRecord>) -> Self {
        Self(values)
    }
}

impl SelectManyLocationsResponse {
    pub fn as_inner(&self) -> &Vec<SelectManyLocationResponseItem> {
        &self.0
    }
}

pub struct InsertOneLocationRequest {
    pub id: crate::entity::location::id::LocationId,
    pub name: String,
    pub r#type: crate::entity::location::r#type::LocationType,
    pub lat: f64,
    pub lng: f64,
}

pub struct InsertOneLocationResponse(LocationRecord);

impl From<LocationRecord> for InsertOneLocationResponse {
    fn from(value: LocationRecord) -> Self {
        Self(value)
    }
}

impl InsertOneLocationResponse {
    pub fn as_inner(&self) -> &LocationRecord {
        &self.0
    }
}

pub struct UpdateOneLocationRequest {
    pub name: String,
    pub r#type: crate::entity::location::r#type::LocationType,
    pub lat: f64,
    pub lng: f64,
}

pub struct UpdateOneLocationResponse(LocationRecord);

impl From<LocationRecord> for UpdateOneLocationResponse {
    fn from(value: LocationRecord) -> Self {
        Self(value)
    }
}

impl UpdateOneLocationResponse {
    pub fn as_inner(&self) -> &LocationRecord {
        &self.0
    }
}

pub struct DeleteOneLocationResponse(LocationRecord);

impl From<LocationRecord> for DeleteOneLocationResponse {
    fn from(value: LocationRecord) -> Self {
        Self(value)
    }
}

impl DeleteOneLocationResponse {
    pub fn as_inner(&self) -> &LocationRecord {
        &self.0
    }
}

pub trait DbLocationAdapter: Clone + Send + Sync + 'static {
    fn select_one_location_by_id(
        &self,
        id: crate::entity::location::id::LocationId,
    ) -> impl ::std::future::Future<Output = Result<SelectOneLocationResponse, crate::error::Failure>>
           + Send;

    fn select_many_locations(
        &self,
    ) -> impl ::std::future::Future<
        Output = Result<SelectManyLocationsResponse, crate::error::Failure>,
    > + Send;

    fn insert_one_location(
        &self,
        location: InsertOneLocationRequest,
    ) -> impl ::std::future::Future<Output = Result<InsertOneLocationResponse, crate::error::Failure>>
           + Send;

    fn update_one_location_by_id(
        &self,
        id: crate::entity::location::id::LocationId,
        location: UpdateOneLocationRequest,
    ) -> impl ::std::future::Future<Output = Result<UpdateOneLocationResponse, crate::error::Failure>>
           + Send;

    fn delete_one_location_by_id(
        &self,
        id: crate::entity::location::id::LocationId,
    ) -> impl ::std::future::Future<Output = Result<DeleteOneLocationResponse, crate::error::Failure>>
           + Send;
}
