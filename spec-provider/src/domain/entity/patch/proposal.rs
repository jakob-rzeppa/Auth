use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entity::patch::Patch;

#[derive(Clone, Copy)]
pub enum ProposalStatus {
    Draft,
    Ready,
}

pub struct Proposal {
    id: Uuid,

    proposed_by: Uuid,
    created_at: DateTime<Utc>,
    last_updated_at: DateTime<Utc>,

    status: ProposalStatus,

    content_uri: String,
}

pub enum ProposalError {
    ProposalNotReady,
}

impl Proposal {
    pub fn new(proposed_by: Uuid, content_uri: String) -> Proposal {
        Proposal {
            id: Uuid::new_v4(),
            proposed_by,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            status: ProposalStatus::Draft,
            content_uri,
        }
    }

    pub fn set_status(&mut self, status: ProposalStatus) {
        self.status = status;
        self.last_updated_at = Utc::now();
    }

    pub fn register_update(&mut self) {
        self.last_updated_at = Utc::now();
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.status, ProposalStatus::Ready)
    }

    pub fn into_patch(self, accepted_by: Uuid) -> Result<Patch, ProposalError> {
        if !matches!(self.status, ProposalStatus::Ready) {
            return Err(ProposalError::ProposalNotReady);
        }

        Ok(Patch {
            id: self.id,
            proposed_by: self.proposed_by,
            created_at: self.created_at,
            accepted_by,
            accepted_at: Utc::now(),
            content_uri: self.content_uri,
        })
    }

    pub fn reconstitute(
        id: Uuid,
        proposed_by: Uuid,
        created_at: DateTime<Utc>,
        last_updated_at: DateTime<Utc>,
        status: ProposalStatus,
        content_uri: String,
    ) -> Proposal {
        Proposal {
            id,
            proposed_by,
            created_at,
            last_updated_at,
            status,
            content_uri,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn proposed_by(&self) -> &Uuid {
        &self.proposed_by
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn last_updated_at(&self) -> &DateTime<Utc> {
        &self.last_updated_at
    }

    pub fn status(&self) -> &ProposalStatus {
        &self.status
    }

    pub fn content_uri(&self) -> &str {
        &self.content_uri
    }
}
