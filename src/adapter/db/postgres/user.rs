use anyhow::Context;

impl crate::service::db::user::DbUserAdapter for crate::adapter::db::postgres::PostgresAdapter {
    async fn select_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
    ) -> Result<crate::service::db::user::SelectOneUserResponse, crate::error::Failure> {
        let a = sqlx::query_as!(
            crate::service::db::user::SelectOneUser,
            r#"
            SELECT id, name, created_at, updated_at
            FROM users
            WHERE id = $1
        "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user by id from DB")?
        .map(crate::service::db::user::SelectOneUser::from)
        .ok_or_else(|| crate::error::Failure::reject_not_found("No user found in DB"))?;

        Ok(a.into())
    }

    async fn select_many_users(
        &self,
    ) -> Result<crate::service::db::user::SelectManyUsersResponse, crate::error::Failure> {
        todo!()
    }

    async fn insert_one_user(
        &self,
        user: crate::service::db::user::InsertOneUserRequest,
    ) -> Result<crate::service::db::user::InsertOneUserResponse, crate::error::Failure> {
        todo!()
    }

    async fn update_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
        user: crate::service::db::user::UpdateOneUserRequest,
    ) -> Result<crate::service::db::user::UpdateOneUserResponse, crate::error::Failure> {
        todo!()
    }

    async fn delete_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
    ) -> Result<crate::service::db::user::DeleteOneUserResponse, crate::error::Failure> {
        todo!()
    }
}
