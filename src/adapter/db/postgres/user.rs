use anyhow::Context;

impl crate::service::db::user::DbUserAdapter for crate::adapter::db::postgres::PostgresAdapter {
    async fn select_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
    ) -> Result<crate::service::db::user::SelectOneUserResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::user::UserRecord,
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
        .map(crate::service::db::user::UserRecord::from)
        .ok_or_else(|| crate::error::Failure::reject_not_found("No user found in DB"))?;

        Ok(res.into())
    }

    async fn select_many_users(
        &self,
    ) -> Result<crate::service::db::user::SelectManyUsersResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::user::UserRecord,
            r#"
            SELECT id, name, created_at, updated_at
            FROM users
        "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch users from DB")?
        .into_iter()
        .map(crate::service::db::user::UserRecord::from)
        .collect::<Vec<_>>();

        Ok(res.into())
    }

    async fn insert_one_user(
        &self,
        user: crate::service::db::user::InsertOneUserRequest,
    ) -> Result<crate::service::db::user::InsertOneUserResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::user::UserRecord,
            r#"
            INSERT INTO users (name) VALUES ($1) RETURNING id, name, created_at, updated_at
        "#,
            user.name
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert user into DB")?;

        Ok(res.into())
    }

    async fn update_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
        user: crate::service::db::user::UpdateOneUserRequest,
    ) -> Result<crate::service::db::user::UpdateOneUserResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::user::UserRecord,
            r#"
            UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name, created_at, updated_at
        "#,
            user.name,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update user in DB")?;

        Ok(res.into())
    }

    async fn delete_one_user_by_id(
        &self,
        id: crate::entity::user::id::UserId,
    ) -> Result<crate::service::db::user::DeleteOneUserResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::user::UserRecord,
            r#"
            DELETE FROM users WHERE id = $1 RETURNING id, name, created_at, updated_at
        "#,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to delete user in DB")?;

        Ok(res.into())
    }
}
