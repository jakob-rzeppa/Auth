use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod proposal;

#[derive(Clone)]
pub struct Patch {
    id: Uuid,

    proposed_by: Uuid,
    created_at: DateTime<Utc>,

    accepted_by: Uuid,
    accepted_at: DateTime<Utc>,

    content_uri: String,
}

impl Patch {
    pub fn reconstitute(
        id: Uuid,
        proposed_by: Uuid,
        created_at: DateTime<Utc>,
        accepted_by: Uuid,
        accepted_at: DateTime<Utc>,
        content_uri: String,
    ) -> Patch {
        Patch {
            id,
            proposed_by,
            created_at,
            accepted_by,
            accepted_at,
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

    pub fn accepted_by(&self) -> &Uuid {
        &self.accepted_by
    }

    pub fn accepted_at(&self) -> &DateTime<Utc> {
        &self.accepted_at
    }

    pub fn content_uri(&self) -> &str {
        &self.content_uri
    }
}
