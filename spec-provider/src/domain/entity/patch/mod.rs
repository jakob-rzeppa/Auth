use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod proposal;

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
}
