use super::{error::OrmError, resource::Resource};

#[async_trait::async_trait]
pub trait ResourceOperations<M, AM, I>
where
    M: Send + Sync,
    AM: Resource + Send + Sync,
    I: Send + Sync,
{
    async fn insert(&self, model: &M) -> Result<AM, OrmError>;

    async fn get(&self, ids: Option<&Vec<I>>) -> Result<Vec<AM>, OrmError>;

    async fn search(&self, domain: &str) -> Result<Vec<AM>, OrmError>;

    async fn save(&self, model: &AM) -> Result<u64, OrmError>;

    async fn delete(&self, id: I) -> Result<u64, OrmError>;

    async fn archive(&self, id: I) -> Result<u64, OrmError>;

    async fn unarchive(&self, id: I) -> Result<u64, OrmError>;
}
