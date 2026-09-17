use uuid::Uuid;

use crate::{domain::entity::client::Client, persistence::clients::store::clients};

pub fn find_client_by_id(id: &Uuid) -> Option<Client> {
    let clients = clients();
    clients.iter().find(|client| client.id() == id).cloned()
}
