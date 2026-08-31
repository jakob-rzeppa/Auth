use uuid::Uuid;

use crate::{
    domain::privilege::{Privilege, PrivilegeError},
    persistence::{DatabaseError, privileges::register::register_privilege},
};

pub enum ApplicationPrivilegeError {
    DomainError(PrivilegeError),
    DatabaseError(DatabaseError),
}

pub async fn create_privilege(
    name: &str,
    description: &str,
) -> Result<Uuid, ApplicationPrivilegeError> {
    let privilege = Privilege::new(Uuid::new_v4(), name.to_string(), description.to_string())
        .map_err(ApplicationPrivilegeError::DomainError)?;

    register_privilege(&privilege)
        .await
        .map_err(ApplicationPrivilegeError::DatabaseError)?;

    Ok(*privilege.id())
}
