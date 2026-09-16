use crate::{domain::entity::client::Client, persistence::clients::store::clients};

pub fn find_client_by_id(id: &str) -> Option<Client> {
    let clients = clients();
    clients
        .iter()
        .find(|client| client.id().to_string() == id)
        .cloned()
}
