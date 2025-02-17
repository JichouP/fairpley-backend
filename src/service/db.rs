pub mod user;

pub trait DbAdapter: Clone + Send + Sync + 'static {
    fn try_new_connection(
        url: &str,
    ) -> impl ::std::future::Future<Output = anyhow::Result<Self>> + Send;
    fn migrate(&self) -> impl ::std::future::Future<Output = anyhow::Result<()>> + Send;
}
