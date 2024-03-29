use async_trait::async_trait;

use crate::{
    domain::entity::{
        external_account::account_id::AccountId, external_entity::entity_code::EntityCode,
    },
    infrastructure::persistence::context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::RepositoryOperations,
    },
    resource::{account_engine::AccountEngine, external},
    shared_kernel::{ids::ExternalEntityId, ArrayString64},
    Store,
};

use super::error::ServiceError;

#[async_trait]
pub trait ExternalService<R>
where
    R: Store
        + RepositoryOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + RepositoryOperations<
            external::entity::Model,
            external::entity::ActiveModel,
            ExternalEntityId,
        > + RepositoryOperations<
            external::entity_type::Model,
            external::entity_type::ActiveModel,
            EntityCode,
        > + Send
        + Sync,
{
    fn store(&self) -> &R;

    async fn create_entity_type(
        &self,
        builder: EntityTypeBuilder,
    ) -> Result<ExternalEntityType, ServiceError> {
        Ok(ExternalEntityType(self.store().insert(&builder.0).await?))
    }

    async fn create_entity(
        &self,
        builder: ExternalEntityBuilder,
    ) -> Result<ExternalEntity, ServiceError> {
        Ok(ExternalEntity(self.store().insert(&builder.0).await?))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EntityTypeBuilder(external::entity_type::Model);

#[derive(Clone, Copy, Debug)]
pub struct ExternalEntityType(external::entity_type::ActiveModel);

#[derive(Clone, Copy, Debug)]
pub struct ExternalEntity(external::entity::ActiveModel);

#[derive(Clone, Copy, Debug)]
pub struct ExternalEntityBuilder(external::entity::Model);

impl EntityTypeBuilder {
    pub fn new(code: EntityCode, description: ArrayString64) -> Self {
        let typ = external::entity_type::Model { code, description };

        Self(typ)
    }
}

impl ExternalEntityType {
    pub fn code(&self) -> EntityCode {
        self.0.code
    }

    pub fn description(&self) -> ArrayString64 {
        self.0.description
    }
}

impl ExternalEntityBuilder {
    pub fn new(code: EntityCode, name: ArrayString64) -> Self {
        let typ = external::entity::Model {
            entity_type_code: code,
            name,
        };

        Self(typ)
    }
}

impl ExternalEntity {
    pub fn id(&self) -> ExternalEntityId {
        self.0.id
    }

    pub fn entity_type_code(&self) -> EntityCode {
        self.0.entity_type_code
    }

    pub fn name(&self) -> ArrayString64 {
        self.0.name
    }
}

#[async_trait]
impl ExternalService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}

#[async_trait]
impl ExternalService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}
