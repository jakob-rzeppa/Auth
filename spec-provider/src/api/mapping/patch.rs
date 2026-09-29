use crate::{api::mapping::to_timestamp, domain::projection::patch::PatchProjection};

impl From<PatchProjection> for spec_api::Patch {
    fn from(patch: PatchProjection) -> Self {
        Self {
            id: patch.id.to_string(),
            proposed_by: patch.proposed_by.to_string(),
            created_at: Some(to_timestamp(patch.created_at)),
            accepted_by: patch.accepted_by.to_string(),
            accepted_at: Some(to_timestamp(patch.accepted_at)),
            content_uri: patch.content_uri,
        }
    }
}
