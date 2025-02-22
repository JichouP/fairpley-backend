use anyhow::Context;

#[derive(sqlx::FromRow)]
struct PurchaseRecordRaw {
    id: String,
    user_id: String,
    user_name: String,
    user_created_at: chrono::DateTime<chrono::Utc>,
    user_updated_at: chrono::DateTime<chrono::Utc>,
    event_id: String,
    event_name: String,
    event_started_at: chrono::DateTime<chrono::Utc>,
    event_ended_at: chrono::DateTime<chrono::Utc>,
    event_created_at: chrono::DateTime<chrono::Utc>,
    event_updated_at: chrono::DateTime<chrono::Utc>,
    name: String,
    price: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl PurchaseRecordRaw {
    fn into_record(
        self,
    ) -> Result<crate::service::db::purchase::PurchaseRecord, crate::error::Failure> {
        let user = crate::entity::user::User {
            id: crate::entity::user::id::UserId::new(
                uuid::Uuid::parse_str(&self.user_id).map_err(|e| anyhow::anyhow!(e))?,
            ),
            name: self.user_name,
            created_at: self.user_created_at,
            updated_at: self.user_updated_at,
        };

        let event = crate::entity::event::Event {
            id: crate::entity::event::id::EventId::new(
                uuid::Uuid::parse_str(&self.event_id).map_err(|e| anyhow::anyhow!(e))?,
            ),
            name: self.event_name,
            started_at: self.event_started_at,
            ended_at: self.event_ended_at,
            created_at: self.event_created_at,
            updated_at: self.event_updated_at,
        };

        Ok(crate::service::db::purchase::PurchaseRecord {
            id: crate::entity::purchase::id::PurchaseId::new(
                uuid::Uuid::parse_str(&self.id).map_err(|e| anyhow::anyhow!(e))?,
            ),
            user,
            event,
            name: self.name,
            price: self.price,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

impl crate::service::db::purchase::DbPurchaseAdapter
    for crate::adapter::db::postgres::PostgresAdapter
{
    async fn select_one_purchase_by_id(
        &self,
        id: crate::entity::purchase::id::PurchaseId,
    ) -> Result<crate::service::db::purchase::SelectOnePurchaseResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            PurchaseRecordRaw,
            r#"
            SELECT 
                p.id,
                u.id as user_id,
                u.name as user_name,
                u.created_at as user_created_at,
                u.updated_at as user_updated_at,
                e.id as event_id,
                e.name as event_name,
                e.started_at as event_started_at,
                e.ended_at as event_ended_at,
                e.created_at as event_created_at,
                e.updated_at as event_updated_at,
                p.name,
                p.price,
                p.created_at,
                p.updated_at
            FROM purchases p
            INNER JOIN users u ON p.user_id = u.id
            INNER JOIN events e ON p.event_id = e.id
            WHERE p.id = $1
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch purchase by id from DB")?
        .map(|raw| raw.into_record())
        .transpose()?
        .ok_or_else(|| crate::error::Failure::reject_not_found("No purchase found in DB"))?;

        Ok(res.into())
    }

    async fn select_many_purchases(
        &self,
    ) -> Result<crate::service::db::purchase::SelectManyPurchasesResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            PurchaseRecordRaw,
            r#"
            SELECT 
                p.id,
                u.id as user_id,
                u.name as user_name,
                u.created_at as user_created_at,
                u.updated_at as user_updated_at,
                e.id as event_id,
                e.name as event_name,
                e.started_at as event_started_at,
                e.ended_at as event_ended_at,
                e.created_at as event_created_at,
                e.updated_at as event_updated_at,
                p.name,
                p.price,
                p.created_at,
                p.updated_at
            FROM purchases p
            INNER JOIN users u ON p.user_id = u.id
            INNER JOIN events e ON p.event_id = e.id
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch purchases from DB")?
        .into_iter()
        .map(|raw| raw.into_record())
        .collect::<Result<Vec<_>, _>>()?;

        Ok(res.into())
    }

    async fn insert_one_purchase(
        &self,
        purchase: crate::service::db::purchase::InsertOnePurchaseRequest,
    ) -> Result<crate::service::db::purchase::InsertOnePurchaseResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            PurchaseRecordRaw,
            r#"
            WITH inserted AS (
                INSERT INTO purchases (id, user_id, event_id, name, price)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, user_id, event_id, name, price, created_at, updated_at
            )
            SELECT 
                p.id,
                u.id as user_id,
                u.name as user_name,
                u.created_at as user_created_at,
                u.updated_at as user_updated_at,
                e.id as event_id,
                e.name as event_name,
                e.started_at as event_started_at,
                e.ended_at as event_ended_at,
                e.created_at as event_created_at,
                e.updated_at as event_updated_at,
                p.name,
                p.price,
                p.created_at,
                p.updated_at
            FROM inserted p
            INNER JOIN users u ON p.user_id = u.id
            INNER JOIN events e ON p.event_id = e.id
            "#,
            purchase.id.into_inner(),
            purchase.user_id.into_inner(),
            purchase.event_id.into_inner(),
            "購入品目", // TODO: Add name field to request
            purchase.amount
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert purchase into DB")?
        .into_record()?;

        Ok(res.into())
    }

    async fn update_one_purchase_by_id(
        &self,
        id: crate::entity::purchase::id::PurchaseId,
        purchase: crate::service::db::purchase::UpdateOnePurchaseRequest,
    ) -> Result<crate::service::db::purchase::UpdateOnePurchaseResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            PurchaseRecordRaw,
            r#"
            WITH updated AS (
                UPDATE purchases
                SET price = $1
                WHERE id = $2
                RETURNING id, user_id, event_id, name, price, created_at, updated_at
            )
            SELECT 
                p.id,
                u.id as user_id,
                u.name as user_name,
                u.created_at as user_created_at,
                u.updated_at as user_updated_at,
                e.id as event_id,
                e.name as event_name,
                e.started_at as event_started_at,
                e.ended_at as event_ended_at,
                e.created_at as event_created_at,
                e.updated_at as event_updated_at,
                p.name,
                p.price,
                p.created_at,
                p.updated_at
            FROM updated p
            INNER JOIN users u ON p.user_id = u.id
            INNER JOIN events e ON p.event_id = e.id
            "#,
            purchase.amount,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update purchase in DB")?
        .into_record()?;

        Ok(res.into())
    }

    async fn delete_one_purchase_by_id(
        &self,
        id: crate::entity::purchase::id::PurchaseId,
    ) -> Result<crate::service::db::purchase::DeleteOnePurchaseResponse, crate::error::Failure>
    {
        let res = sqlx::query_as!(
            PurchaseRecordRaw,
            r#"
            WITH deleted AS (
                DELETE FROM purchases
                WHERE id = $1
                RETURNING id, user_id, event_id, name, price, created_at, updated_at
            )
            SELECT 
                p.id,
                u.id as user_id,
                u.name as user_name,
                u.created_at as user_created_at,
                u.updated_at as user_updated_at,
                e.id as event_id,
                e.name as event_name,
                e.started_at as event_started_at,
                e.ended_at as event_ended_at,
                e.created_at as event_created_at,
                e.updated_at as event_updated_at,
                p.name,
                p.price,
                p.created_at,
                p.updated_at
            FROM deleted p
            INNER JOIN users u ON p.user_id = u.id
            INNER JOIN events e ON p.event_id = e.id
            "#,
            id.into_inner()
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to delete purchase from DB")?
        .into_record()?;

        Ok(res.into())
    }
}
