use anyhow::Context;
use uuid;

#[derive(sqlx::FromRow)]
struct LocationRecordRaw {
    id: String,
    name: String,
    r#type: String,
    lat: f64,
    lng: f64,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl LocationRecordRaw {
    fn into_record(
        self,
    ) -> Result<crate::service::db::location::LocationRecord, crate::error::Failure> {
        let location_type =
            crate::entity::location::r#type::LocationType::try_from(self.r#type.as_str())
                .map_err(|_| crate::error::Failure::reject_bad_request("Invalid location type"))?;

        Ok(crate::service::db::location::LocationRecord {
            id: crate::entity::location::id::LocationId::new(
                uuid::Uuid::parse_str(&self.id).map_err(|e| anyhow::anyhow!(e))?,
            ),
            name: self.name,
            r#type: location_type,
            lat: self.lat,
            lng: self.lng,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

impl crate::service::db::location::DbLocationAdapter
    for crate::adapter::db::postgres::PostgresAdapter
{
    async fn select_one_location_by_id(
        &self,
        id: crate::entity::location::id::LocationId,
    ) -> Result<crate::service::db::location::SelectOneLocationResponse, crate::error::Failure>
    {
        let raw = sqlx::query_as!(
            LocationRecordRaw,
            r#"
            SELECT id, name, type, lat, lng, created_at, updated_at
            FROM locations
            WHERE id = $1
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch location by id from DB")?
        .ok_or_else(|| crate::error::Failure::reject_not_found("No location found in DB"))?;

        Ok(raw.into_record()?.into())
    }

    async fn select_many_locations(
        &self,
    ) -> Result<crate::service::db::location::SelectManyLocationsResponse, crate::error::Failure>
    {
        let raw_records = sqlx::query_as!(
            LocationRecordRaw,
            r#"
            SELECT id, name, type, lat, lng, created_at, updated_at
            FROM locations
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch locations from DB")?;

        let records = raw_records
            .into_iter()
            .map(|raw| raw.into_record())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records.into())
    }

    async fn insert_one_location(
        &self,
        location: crate::service::db::location::InsertOneLocationRequest,
    ) -> Result<crate::service::db::location::InsertOneLocationResponse, crate::error::Failure>
    {
        let raw = sqlx::query_as!(
            LocationRecordRaw,
            r#"
            INSERT INTO locations (id, name, type, lat, lng)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, type, lat, lng, created_at, updated_at
            "#,
            location.id.into_inner(),
            location.name,
            location.r#type.as_ref(),
            location.lat,
            location.lng
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert location into DB")?;

        Ok(raw.into_record()?.into())
    }

    async fn update_one_location_by_id(
        &self,
        id: crate::entity::location::id::LocationId,
        location: crate::service::db::location::UpdateOneLocationRequest,
    ) -> Result<crate::service::db::location::UpdateOneLocationResponse, crate::error::Failure>
    {
        let raw = sqlx::query_as!(
            LocationRecordRaw,
            r#"
            UPDATE locations
            SET name = $1, type = $2, lat = $3, lng = $4
            WHERE id = $5
            RETURNING id, name, type, lat, lng, created_at, updated_at
            "#,
            location.name,
            location.r#type.as_ref(),
            location.lat,
            location.lng,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update location in DB")?;

        Ok(raw.into_record()?.into())
    }

    async fn delete_one_location_by_id(
        &self,
        id: crate::entity::location::id::LocationId,
    ) -> Result<crate::service::db::location::DeleteOneLocationResponse, crate::error::Failure>
    {
        let raw = sqlx::query_as!(
            LocationRecordRaw,
            r#"
            DELETE FROM locations
            WHERE id = $1
            RETURNING id, name, type, lat, lng, created_at, updated_at
            "#,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to delete location from DB")?;

        Ok(raw.into_record()?.into())
    }
}
