pub type UserRecord = crate::entity::user::User;

pub struct SelectOneUserResponse(UserRecord);

impl From<UserRecord> for SelectOneUserResponse {
    fn from(value: UserRecord) -> Self {
        Self(value)
    }
}

impl SelectOneUserResponse {
    pub fn as_inner(&self) -> &UserRecord {
        &self.0
    }
}

pub type SelectManyUserResponseItem = UserRecord;

pub struct SelectManyUsersResponse(Vec<SelectManyUserResponseItem>);

impl From<Vec<UserRecord>> for SelectManyUsersResponse {
    fn from(values: Vec<UserRecord>) -> Self {
        Self(values)
    }
}

impl SelectManyUsersResponse {
    pub fn as_inner(&self) -> &Vec<SelectManyUserResponseItem> {
        &self.0
    }
}

pub struct InsertOneUserRequest {
    pub id: crate::entity::user::id::UserId,
    pub name: String,
}

pub struct InsertOneUserResponse(UserRecord);

impl From<UserRecord> for InsertOneUserResponse {
    fn from(value: UserRecord) -> Self {
        Self(value)
    }
}

impl InsertOneUserResponse {
    pub fn as_inner(&self) -> &UserRecord {
        &self.0
    }
}

pub struct UpdateOneUserRequest {
    pub name: String,
}

pub struct UpdateOneUserResponse(UserRecord);

impl From<UserRecord> for UpdateOneUserResponse {
    fn from(value: UserRecord) -> Self {
        Self(value)
    }
}

impl UpdateOneUserResponse {
    pub fn as_inner(&self) -> &UserRecord {
        &self.0
    }
}

pub struct DeleteOneUserResponse(UserRecord);

impl From<UserRecord> for DeleteOneUserResponse {
    fn from(value: UserRecord) -> Self {
        Self(value)
    }
}

impl DeleteOneUserResponse {
    pub fn as_inner(&self) -> &UserRecord {
        &self.0
    }
}

pub trait DbUserAdapter: Clone + Send + Sync + 'static {
    fn select_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
    ) -> impl ::std::future::Future<Output = Result<SelectOneUserResponse, crate::error::Failure>> + Send;
    fn select_many_users(
        &self,
    ) -> impl ::std::future::Future<Output = Result<SelectManyUsersResponse, crate::error::Failure>> + Send;
    fn insert_one_user(
        &self,
        user: InsertOneUserRequest,
    ) -> impl ::std::future::Future<Output = Result<InsertOneUserResponse, crate::error::Failure>> + Send;
    fn update_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
        user: UpdateOneUserRequest,
    ) -> impl ::std::future::Future<Output = Result<UpdateOneUserResponse, crate::error::Failure>> + Send;
    fn delete_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
    ) -> impl ::std::future::Future<Output = Result<DeleteOneUserResponse, crate::error::Failure>> + Send;
}
