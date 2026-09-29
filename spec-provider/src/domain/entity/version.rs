use crate::domain::entity::patch::Patch;

#[derive(Clone)]
pub struct Version {
    id: uuid::Uuid,

    patches: Vec<Patch>,
}

impl Version {
    pub fn new(patches: Vec<Patch>) -> Version {
        Version {
            id: uuid::Uuid::new_v4(),
            patches,
        }
    }

    pub fn reconstitute(id: uuid::Uuid, patches: Vec<Patch>) -> Version {
        Version { id, patches }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn patches(&self) -> &[Patch] {
        &self.patches
    }
}
