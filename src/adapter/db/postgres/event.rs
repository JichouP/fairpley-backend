use anyhow::Context;

impl crate::service::db::event::DbEventAdapter for crate::adapter::db::postgres::PostgresAdapter {
    async fn select_one_event_by_id(
        &self,
        id: crate::entity::event::id::EventId,
    ) -> Result<crate::service::db::event::SelectOneEventResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::event::EventRecord,
            r#"
            SELECT id, name, started_at, ended_at, created_at, updated_at
            FROM events
            WHERE id = $1
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch event by id from DB")?
        .map(crate::service::db::event::EventRecord::from)
        .ok_or_else(|| crate::error::Failure::reject_not_found("No event found in DB"))?;

        Ok(res.into())
    }

    async fn select_many_events(
        &self,
    ) -> Result<crate::service::db::event::SelectManyEventsResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::event::EventRecord,
            r#"
            SELECT id, name, started_at, ended_at, created_at, updated_at
            FROM events
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch events from DB")?
        .into_iter()
        .map(crate::service::db::event::EventRecord::from)
        .collect::<Vec<_>>();

        Ok(res.into())
    }

    async fn insert_one_event(
        &self,
        event: crate::service::db::event::InsertOneEventRequest,
    ) -> Result<crate::service::db::event::InsertOneEventResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::event::EventRecord,
            r#"
            INSERT INTO events (id, name, started_at, ended_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, started_at, ended_at, created_at, updated_at
            "#,
            event.id.into_inner(),
            event.name,
            event.started_at,
            event.ended_at
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert event into DB")?;

        Ok(res.into())
    }

    async fn update_one_event_by_id(
        &self,
        id: crate::entity::event::id::EventId,
        event: crate::service::db::event::UpdateOneEventRequest,
    ) -> Result<crate::service::db::event::UpdateOneEventResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::event::EventRecord,
            r#"
            UPDATE events
            SET name = $1, started_at = $2, ended_at = $3
            WHERE id = $4
            RETURNING id, name, started_at, ended_at, created_at, updated_at
            "#,
            event.name,
            event.started_at,
            event.ended_at,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update event in DB")?;

        Ok(res.into())
    }

    async fn delete_one_event_by_id(
        &self,
        id: crate::entity::event::id::EventId,
    ) -> Result<crate::service::db::event::DeleteOneEventResponse, crate::error::Failure> {
        let res = sqlx::query_as!(
            crate::service::db::event::EventRecord,
            r#"
            DELETE FROM events
            WHERE id = $1
            RETURNING id, name, started_at, ended_at, created_at, updated_at
            "#,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to delete event from DB")?;

        Ok(res.into())
    }
}
