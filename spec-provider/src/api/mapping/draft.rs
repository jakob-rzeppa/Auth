use crate::domain::{entity::spec::draft::DraftVisibility, projection::draft::DraftProjection};

impl From<DraftVisibility> for spec_api::DraftVisibility {
    fn from(visibility: DraftVisibility) -> Self {
        match visibility {
            DraftVisibility::Public => Self::Public,
            DraftVisibility::Private => Self::Private,
            DraftVisibility::Hidden => Self::Hidden,
        }
    }
}

impl From<DraftProjection> for spec_api::Draft {
    fn from(draft: DraftProjection) -> Self {
        Self {
            id: draft.id.to_string(),
            title: draft.title,
            abbreviation: draft.abbreviation,
            visibility: spec_api::DraftVisibility::from(draft.visibility).into(),
            versions: draft.versions.into_iter().map(Into::into).collect(),
            unversioned_patches: draft
                .unversioned_patches
                .into_iter()
                .map(Into::into)
                .collect(),
            proposals: draft.proposals.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;
    use crate::{
        api::mapping::to_timestamp,
        domain::{
            entity::patch::proposal::ProposalStatus,
            projection::{
                patch::PatchProjection, proposal::ProposalProjection, version::VersionProjection,
            },
        },
    };

    #[test]
    fn draft_maps_ids_enums_and_nested_messages() {
        let now = Utc::now();
        let patch = PatchProjection {
            id: Uuid::new_v4(),
            proposed_by: Uuid::new_v4(),
            created_at: now,
            accepted_by: Uuid::new_v4(),
            accepted_at: now,
            content_uri: "patch-uri".to_string(),
        };
        let proposal = ProposalProjection {
            id: Uuid::new_v4(),
            proposed_by: Uuid::new_v4(),
            created_at: now,
            last_updated_at: now,
            status: ProposalStatus::Ready,
            content_uri: "proposal-uri".to_string(),
        };
        let version_id = Uuid::new_v4();
        let draft = DraftProjection {
            id: Uuid::new_v4(),
            title: "Title".to_string(),
            abbreviation: "ABBR".to_string(),
            visibility: DraftVisibility::Private,
            versions: vec![VersionProjection {
                id: version_id,
                patches: vec![],
            }],
            unversioned_patches: vec![patch],
            proposals: vec![proposal],
        };
        let draft_id = draft.id;

        let message = spec_api::Draft::from(draft);

        assert_eq!(message.id, draft_id.to_string());
        assert_eq!(
            message.visibility,
            spec_api::DraftVisibility::Private as i32
        );
        assert_eq!(message.versions[0].id, version_id.to_string());
        assert_eq!(message.unversioned_patches[0].content_uri, "patch-uri");
        assert_eq!(
            message.proposals[0].status,
            spec_api::ProposalStatus::Ready as i32
        );
        assert_eq!(message.proposals[0].created_at, Some(to_timestamp(now)));
    }
}
