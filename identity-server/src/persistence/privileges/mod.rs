use uuid::Uuid;

pub mod register;

struct PrivilegeRow {
    id: Uuid,
    name: String,
    description: String,
}
