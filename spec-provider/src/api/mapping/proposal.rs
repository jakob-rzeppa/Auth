use crate::{
    api::mapping::to_timestamp,
    domain::{entity::patch::proposal::ProposalStatus, projection::proposal::ProposalProjection},
};

impl From<ProposalStatus> for spec_api::ProposalStatus {
    fn from(status: ProposalStatus) -> Self {
        match status {
            ProposalStatus::Draft => Self::Draft,
            ProposalStatus::Ready => Self::Ready,
        }
    }
}

impl From<ProposalProjection> for spec_api::Proposal {
    fn from(proposal: ProposalProjection) -> Self {
        Self {
            id: proposal.id.to_string(),
            proposed_by: proposal.proposed_by.to_string(),
            created_at: Some(to_timestamp(proposal.created_at)),
            last_updated_at: Some(to_timestamp(proposal.last_updated_at)),
            status: spec_api::ProposalStatus::from(proposal.status).into(),
            content_uri: proposal.content_uri,
        }
    }
}
