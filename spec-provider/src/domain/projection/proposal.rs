use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entity::patch::proposal::{Proposal, ProposalStatus};

pub struct ProposalProjection {
    pub id: Uuid,

    pub proposed_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,

    pub status: ProposalStatus,

    pub content_uri: String,
}

impl From<&Proposal> for ProposalProjection {
    fn from(proposal: &Proposal) -> Self {
        Self {
            id: proposal.id().clone(),
            proposed_by: proposal.proposed_by().clone(),
            created_at: proposal.created_at().clone(),
            last_updated_at: proposal.last_updated_at().clone(),
            status: proposal.status().clone(),
            content_uri: proposal.content_uri().to_string(),
        }
    }
}
