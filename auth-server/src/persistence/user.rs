use crate::{domain::user::User, persistence::PersistanceError};

pub fn get_user_by_email(email: &str) -> Result<Option<User>, PersistanceError> {
    todo!();
}
