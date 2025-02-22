use anyhow::Context;

pub mod event;
pub mod location;
pub mod purchase;
pub mod transport;
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
            .context("Failed to connect to Postgres")?;

        tracing::info!("Connected to Postgres: {:?}", &pool);

        Ok(Self::new(pool))
    }

    async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to migrate Postgres")?;

        tracing::info!("Migration completed");

        Ok(())
    }
}
