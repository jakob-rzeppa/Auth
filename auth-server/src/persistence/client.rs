use uuid::Uuid;

use crate::{domain::client::Client, persistence::PersistanceError};

pub fn get_client_by_id(id: &Uuid) -> Result<Option<Client>, PersistanceError> {
    todo!();
}
