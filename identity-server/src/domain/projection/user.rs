use privilege::Privilege;
use serde::Serialize;

use crate::domain::entity::user::User;

#[derive(Serialize)]
pub struct FullUserProjection {
    pub id: String,
    pub email: String,

    pub privileges: Vec<Privilege>,
}

impl From<User> for FullUserProjection {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            email: user.email().to_string(),
            privileges: user.privileges().to_vec(),
        }
    }
}
