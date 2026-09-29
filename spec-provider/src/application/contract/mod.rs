pub mod draft_repository;
pub mod spec_repository;

#[derive(Debug, PartialEq)]
pub enum RepositoryError {
    /// `register` was called with an ID that already exists.
    AlreadyExists,
    /// `save` or `delete_by_id` was called with an ID that doesn't exist.
    NotFound,
    DatabaseError,
}
