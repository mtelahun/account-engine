use async_trait::async_trait;
use chrono::NaiveDate;

use crate::{
    infrastructure::persistence::context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::RepositoryOperations,
    },
    resource::{account_engine::AccountEngine, external},
    shared_kernel::{ids::ExternalEntityId, ArrayString128, ArrayString24},
    Store,
};

use super::{
    entity::{
        external_account::account_id::AccountId, external_entity::entity_code::EntityCode,
        subsidiary_ledger::subleder_id::SubLedgerId,
    },
    ServiceError,
};

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
pub struct ExternalAccountBuilder(external::account::Model);

#[derive(Clone, Copy, Debug)]
pub struct ExternalAccount(external::account::ActiveModel);

#[derive(Clone, Copy, Debug)]
pub struct EntityTypeBuilder(external::entity_type::Model);

#[derive(Clone, Copy, Debug)]
pub struct ExternalEntityType(external::entity_type::ActiveModel);

#[derive(Clone, Copy, Debug)]
pub struct ExternalEntity(external::entity::ActiveModel);

#[derive(Clone, Copy, Debug)]
pub struct ExternalEntityBuilder(external::entity::Model);

impl ExternalAccountBuilder {
    pub fn new(
        subledger_id: &SubLedgerId,
        entity_id: &ExternalEntityId,
        account_no: ArrayString24,
        name: ArrayString128,
        date_opened: NaiveDate,
    ) -> ExternalAccountBuilder {
        let model = external::account::Model {
            subledger_id: *subledger_id,
            entity_id: *entity_id,
            account_no,
            name,
            date_opened,
        };

        Self(model)
    }

    pub(crate) fn to_model(self) -> external::account::Model {
        self.0
    }
}

impl ExternalAccount {
    pub fn account_no(&self) -> ArrayString24 {
        self.0.account_no
    }

    pub fn date_opened(&self) -> NaiveDate {
        self.0.date_opened
    }

    pub fn entity_id(&self) -> ExternalEntityId {
        self.0.entity_id
    }

    pub fn id(&self) -> AccountId {
        self.0.id
    }

    pub fn name(&self) -> ArrayString128 {
        self.0.name
    }

    pub fn subledger_id(&self) -> SubLedgerId {
        self.0.subledger_id
    }
}

impl EntityTypeBuilder {
    pub fn new(code: EntityCode, description: ArrayString128) -> Self {
        let typ = external::entity_type::Model { code, description };

        Self(typ)
    }
}

impl ExternalEntityType {
    pub fn code(&self) -> EntityCode {
        self.0.code
    }

    pub fn description(&self) -> ArrayString128 {
        self.0.description
    }
}

impl ExternalEntityBuilder {
    pub fn new(code: EntityCode, name: ArrayString128) -> Self {
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

    pub fn name(&self) -> ArrayString128 {
        self.0.name
    }
}

impl From<external::account::ActiveModel> for ExternalAccount {
    fn from(value: external::account::ActiveModel) -> Self {
        Self(value)
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
