use privilege::Privilege;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::entity::user::User;

/// `Privilege` lives in an external crate and can't derive `utoipa::ToSchema`
/// itself, so the field is documented as its serialised shape (its name) instead.
#[derive(Serialize, utoipa::ToSchema)]
pub struct FullUserProjection {
    pub id: Uuid,

    pub user_name: String,
    pub display_name: String,

    pub has_temporary_password: bool,

    #[schema(value_type = Vec<String>)]
    pub privileges: Vec<Privilege>,
}

impl From<&User> for FullUserProjection {
    fn from(user: &User) -> Self {
        Self {
            id: user.id(),
            user_name: user.user_name().to_string(),
            display_name: user.display_name().to_string(),
            has_temporary_password: user.has_temporary_password(),
            privileges: user.privileges().to_vec(),
        }
    }
}
