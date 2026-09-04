use privilege::Privilege;
use serde::Serialize;

use crate::domain::entity::user::User;

/// `Privilege` lives in an external crate and can't derive `utoipa::ToSchema`
/// itself, so the field is documented as its serialised shape (its name) instead.
#[derive(Serialize, utoipa::ToSchema)]
pub struct FullUserProjection {
    pub id: String,
    pub email: String,

    #[schema(value_type = Vec<String>)]
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
