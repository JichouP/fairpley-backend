use anyhow::Context;

pub mod user;

#[derive(Clone)]
pub struct PostgresAdapter {
    pool: sqlx::PgPool,
}

impl PostgresAdapter {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl crate::service::db::DbAdapter for PostgresAdapter {
    async fn try_new_connection(url: &str) -> anyhow::Result<Self> {
        let pool = sqlx::PgPool::connect(url)
            .await
            .context(crate::error::messages::FAILED_TO_CONNECT_TO_POSTGRES)?;

        tracing::info!("Connected to Postgres: {:?}", &pool);

        Ok(Self::new(pool))
    }

    async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context(crate::error::messages::FAILED_TO_MIGRATE_POSTGRES)?;

        tracing::info!("Migration completed");

        Ok(())
    }
}
