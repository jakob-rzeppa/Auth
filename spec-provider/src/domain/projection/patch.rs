use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entity::patch::Patch;

pub struct PatchProjection {
    pub id: Uuid,

    pub proposed_by: Uuid,
    pub created_at: DateTime<Utc>,

    pub accepted_by: Uuid,
    pub accepted_at: DateTime<Utc>,

    pub content_uri: String,
}

impl From<&Patch> for PatchProjection {
    fn from(patch: &Patch) -> Self {
        Self {
            id: patch.id().clone(),
            proposed_by: patch.proposed_by().clone(),
            created_at: patch.created_at().clone(),
            accepted_by: patch.accepted_by().clone(),
            accepted_at: patch.accepted_at().clone(),
            content_uri: patch.content_uri().to_string(),
        }
    }
}
