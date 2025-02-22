use anyhow::Context;

#[derive(sqlx::FromRow)]
struct TransportRecordRaw {
    id: String,
    name: String,
    r#type: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl TransportRecordRaw {
    fn into_record(
        self,
    ) -> Result<crate::service::db::transport::TransportRecord, crate::error::Failure> {
        Ok(crate::service::db::transport::TransportRecord {
            id: crate::entity::transport::id::TransportId::new(
                uuid::Uuid::parse_str(&self.id).map_err(|e| anyhow::anyhow!(e))?,
            ),
            name: self.name,
            r#type: self.r#type,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

impl crate::service::db::transport::DbTransportAdapter
    for crate::adapter::db::postgres::PostgresAdapter
{
    async fn select_one_transport_by_id(
        &self,
        id: crate::entity::transport::id::TransportId,
    ) -> Result<crate::service::db::transport::SelectOneTransportResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            TransportRecordRaw,
            r#"
            SELECT 
                id,
                name,
                type,
                created_at,
                updated_at
            FROM transports
            WHERE id = $1
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch transport by id from DB")?
        .map(|raw| raw.into_record())
        .transpose()?
        .ok_or_else(|| crate::error::Failure::reject_not_found("No transport found in DB"))?;

        Ok(res.into())
    }

    async fn select_many_transports(
        &self,
    ) -> Result<crate::service::db::transport::SelectManyTransportsResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            TransportRecordRaw,
            r#"
            SELECT 
                id,
                name,
                type,
                created_at,
                updated_at
            FROM transports
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch transports from DB")?
        .into_iter()
        .map(|raw| raw.into_record())
        .collect::<Result<Vec<_>, _>>()?;

        Ok(res.into())
    }

    async fn insert_one_transport(
        &self,
        transport: crate::service::db::transport::InsertOneTransportRequest,
    ) -> Result<crate::service::db::transport::InsertOneTransportResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            TransportRecordRaw,
            r#"
            INSERT INTO transports (id, name, type)
            VALUES ($1, $2, $3)
            RETURNING 
                id,
                name,
                type,
                created_at,
                updated_at
            "#,
            transport.id.into_inner(),
            "移動手段", // TODO: Add name field to request
            transport.r#type
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert transport into DB")?
        .into_record()?;

        Ok(res.into())
    }

    async fn update_one_transport_by_id(
        &self,
        id: crate::entity::transport::id::TransportId,
        transport: crate::service::db::transport::UpdateOneTransportRequest,
    ) -> Result<crate::service::db::transport::UpdateOneTransportResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            TransportRecordRaw,
            r#"
            UPDATE transports
            SET type = $1
            WHERE id = $2
            RETURNING 
                id,
                name,
                type,
                created_at,
                updated_at
            "#,
            transport.r#type,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update transport in DB")?
        .into_record()?;

        Ok(res.into())
    }

    async fn delete_one_transport_by_id(
        &self,
        id: crate::entity::transport::id::TransportId,
    ) -> Result<crate::service::db::transport::DeleteOneTransportResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            TransportRecordRaw,
            r#"
            DELETE FROM transports
            WHERE id = $1
            RETURNING 
                id,
                name,
                type,
                created_at,
                updated_at
            "#,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to delete transport from DB")?
        .into_record()?;

        Ok(res.into())
    }
}
