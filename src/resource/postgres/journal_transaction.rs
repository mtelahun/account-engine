use crate::{entity::journal, resource::Resource};

/// The journal_transaction::ActiveModel is only ever used to communicate with
/// the caller and doesn't have any datastore models associated with it.
impl Resource for journal::transaction::ActiveModel {
    const NAME: &'static str = "";
}
