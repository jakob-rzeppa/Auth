use crate::domain::projection::spec::SpecProjection;

impl From<SpecProjection> for spec_api::Spec {
    fn from(spec: SpecProjection) -> Self {
        Self {
            id: spec.id.to_string(),
            title: spec.title,
            abbreviation: spec.abbreviation,
            versions: spec.versions.into_iter().map(Into::into).collect(),
        }
    }
}
