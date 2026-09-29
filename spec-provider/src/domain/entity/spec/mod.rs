use crate::domain::entity::version::Version;

pub mod draft;

/// A published spec.
pub struct Spec {
    id: uuid::Uuid,

    /// The title of the spec.
    title: String,
    /// A short, unique abbreviation for the spec.
    abbreviation: String,

    /// The versions of the spec.
    ///
    /// The latest version (last element) is the final version of the spec
    /// that is used to display the spec by default.
    /// The other versions are kept for historical purposes.
    versions: Vec<Version>,
}

impl Spec {
    pub fn reconstitute(
        id: uuid::Uuid,
        title: String,
        abbreviation: String,
        versions: Vec<Version>,
    ) -> Spec {
        Spec {
            id,
            title,
            abbreviation,
            versions,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn abbreviation(&self) -> &str {
        &self.abbreviation
    }

    pub fn versions(&self) -> &[Version] {
        &self.versions
    }
}
