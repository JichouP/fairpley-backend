pub struct SelectOneUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct SelectOneUserResponse(SelectOneUser);

impl From<SelectOneUser> for SelectOneUserResponse {
    fn from(value: SelectOneUser) -> Self {
        Self(value)
    }
}

pub type SelectManyUserResponseItem = SelectOneUser;

pub struct SelectManyUsersResponse(Vec<SelectManyUserResponseItem>);

pub struct InsertOneUserRequest {
    pub id: crate::entity::user::id::UserId,
    pub name: String,
}

pub struct InsertOneUserResponse(SelectOneUser);

pub struct UpdateOneUserRequest {
    pub name: String,
}

pub struct UpdateOneUserResponse(SelectOneUser);

pub struct DeleteOneUserResponse(SelectOneUser);

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
