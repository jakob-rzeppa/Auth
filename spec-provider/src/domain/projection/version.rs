use uuid::Uuid;

use crate::domain::{entity::version::Version, projection::patch::PatchProjection};

pub struct VersionProjection {
    pub id: Uuid,

    pub patches: Vec<PatchProjection>,
}

impl From<&Version> for VersionProjection {
    fn from(version: &Version) -> Self {
        Self {
            id: version.id().clone(),
            patches: version
                .patches()
                .iter()
                .map(PatchProjection::from)
                .collect(),
        }
    }
}
