pub type PurchaseRecord = crate::entity::purchase::Purchase;

pub struct SelectOnePurchaseResponse(PurchaseRecord);

impl From<PurchaseRecord> for SelectOnePurchaseResponse {
    fn from(value: PurchaseRecord) -> Self {
        Self(value)
    }
}

impl SelectOnePurchaseResponse {
    pub fn as_inner(&self) -> &PurchaseRecord {
        &self.0
    }
}

pub type SelectManyPurchaseResponseItem = PurchaseRecord;

pub struct SelectManyPurchasesResponse(Vec<SelectManyPurchaseResponseItem>);

impl From<Vec<PurchaseRecord>> for SelectManyPurchasesResponse {
    fn from(values: Vec<PurchaseRecord>) -> Self {
        Self(values)
    }
}

impl SelectManyPurchasesResponse {
    pub fn as_inner(&self) -> &Vec<SelectManyPurchaseResponseItem> {
        &self.0
    }
}

pub struct InsertOnePurchaseRequest {
    pub id: crate::entity::purchase::id::PurchaseId,
    pub user_id: crate::entity::user::id::UserId,
    pub event_id: crate::entity::event::id::EventId,
    pub amount: i32,
    pub status: String,
}

pub struct InsertOnePurchaseResponse(PurchaseRecord);

impl From<PurchaseRecord> for InsertOnePurchaseResponse {
    fn from(value: PurchaseRecord) -> Self {
        Self(value)
    }
}

impl InsertOnePurchaseResponse {
    pub fn as_inner(&self) -> &PurchaseRecord {
        &self.0
    }
}

pub struct UpdateOnePurchaseRequest {
    pub amount: i32,
    pub status: String,
}

pub struct UpdateOnePurchaseResponse(PurchaseRecord);

impl From<PurchaseRecord> for UpdateOnePurchaseResponse {
    fn from(value: PurchaseRecord) -> Self {
        Self(value)
    }
}

impl UpdateOnePurchaseResponse {
    pub fn as_inner(&self) -> &PurchaseRecord {
        &self.0
    }
}

pub struct DeleteOnePurchaseResponse(PurchaseRecord);

impl From<PurchaseRecord> for DeleteOnePurchaseResponse {
    fn from(value: PurchaseRecord) -> Self {
        Self(value)
    }
}

impl DeleteOnePurchaseResponse {
    pub fn as_inner(&self) -> &PurchaseRecord {
        &self.0
    }
}

pub trait DbPurchaseAdapter: Clone + Send + Sync + 'static {
    fn select_one_purchase_by_id(
        &self,
        id: crate::entity::purchase::id::PurchaseId,
    ) -> impl ::std::future::Future<Output = Result<SelectOnePurchaseResponse, crate::error::Failure>>
           + Send;

    fn select_many_purchases(
        &self,
    ) -> impl ::std::future::Future<
        Output = Result<SelectManyPurchasesResponse, crate::error::Failure>,
    > + Send;

    fn insert_one_purchase(
        &self,
        purchase: InsertOnePurchaseRequest,
    ) -> impl ::std::future::Future<Output = Result<InsertOnePurchaseResponse, crate::error::Failure>>
           + Send;

    fn update_one_purchase_by_id(
        &self,
        id: crate::entity::purchase::id::PurchaseId,
        purchase: UpdateOnePurchaseRequest,
    ) -> impl ::std::future::Future<Output = Result<UpdateOnePurchaseResponse, crate::error::Failure>>
           + Send;

    fn delete_one_purchase_by_id(
        &self,
        id: crate::entity::purchase::id::PurchaseId,
    ) -> impl ::std::future::Future<Output = Result<DeleteOnePurchaseResponse, crate::error::Failure>>
           + Send;
}
