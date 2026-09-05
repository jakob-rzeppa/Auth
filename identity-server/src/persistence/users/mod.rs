use sqlx::prelude::FromRow;
use uuid::Uuid;

pub mod find_by_email;
pub mod find_by_id;
pub mod register;
pub mod remove;
pub mod save;

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    user_name: String,
    display_name: String,
    password_hash: String,
    has_temporary_password: bool,
}
