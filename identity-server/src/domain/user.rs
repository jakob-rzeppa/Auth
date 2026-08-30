use uuid::Uuid;

use crate::domain::privilege::Privilege;

pub struct User {
    id: Uuid,

    email: String,
    /// Hashed and salted password
    password: String,

    privileges: Vec<Privilege>,
}
