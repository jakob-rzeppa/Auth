use uuid::Uuid;

use crate::domain::{entity::spec::Spec, projection::version::VersionProjection};

pub struct SpecProjection {
    pub id: Uuid,

    pub title: String,
    pub abbreviation: String,

    pub versions: Vec<VersionProjection>,
}

impl From<&Spec> for SpecProjection {
    fn from(spec: &Spec) -> Self {
        Self {
            id: spec.id().clone(),
            title: spec.title().to_string(),
            abbreviation: spec.abbreviation().to_string(),
            versions: spec
                .versions()
                .iter()
                .map(VersionProjection::from)
                .collect(),
        }
    }
}
