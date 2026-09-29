use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entity::version::Version;

pub mod draft;

/// A published spec.
#[derive(Clone)]
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

    owners: Vec<Uuid>,
    created_by: Uuid,
    created_at: DateTime<Utc>,
}

impl Spec {
    pub fn reconstitute(
        id: uuid::Uuid,
        title: String,
        abbreviation: String,
        versions: Vec<Version>,
        owners: Vec<Uuid>,
        created_by: Uuid,
        created_at: DateTime<Utc>,
    ) -> Spec {
        Spec {
            id,
            title,
            abbreviation,
            versions,
            owners,
            created_by,
            created_at,
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

    pub fn owners(&self) -> &[Uuid] {
        &self.owners
    }

    pub fn created_by(&self) -> &Uuid {
        &self.created_by
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
