use crate::domain::projection::version::VersionProjection;

impl From<VersionProjection> for spec_api::Version {
    fn from(version: VersionProjection) -> Self {
        Self {
            id: version.id.to_string(),
            patches: version.patches.into_iter().map(Into::into).collect(),
        }
    }
}
