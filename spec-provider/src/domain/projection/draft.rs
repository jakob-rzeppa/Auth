use uuid::Uuid;

use crate::domain::{
    entity::spec::draft::{Draft, DraftVisibility},
    projection::{
        patch::PatchProjection, proposal::ProposalProjection, version::VersionProjection,
    },
};

pub struct DraftProjection {
    pub id: Uuid,

    pub title: String,
    pub abbreviation: String,

    pub visibility: DraftVisibility,

    pub versions: Vec<VersionProjection>,
    pub unversioned_patches: Vec<PatchProjection>,
    pub proposals: Vec<ProposalProjection>,
}

impl From<&Draft> for DraftProjection {
    fn from(draft: &Draft) -> Self {
        Self {
            id: draft.id().clone(),
            title: draft.title().to_string(),
            abbreviation: draft.abbreviation().to_string(),
            visibility: draft.visibility().clone(),
            versions: draft
                .versions()
                .iter()
                .map(VersionProjection::from)
                .collect(),
            unversioned_patches: draft
                .unversioned_patches()
                .iter()
                .map(PatchProjection::from)
                .collect(),
            proposals: draft
                .proposals()
                .iter()
                .map(ProposalProjection::from)
                .collect(),
        }
    }
}
