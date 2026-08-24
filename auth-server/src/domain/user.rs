use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,

    // TODO: this should be hashed and salted
    pub password: String,
}
