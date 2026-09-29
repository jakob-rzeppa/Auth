use uuid::Uuid;

use crate::domain::entity::{
    patch::{
        Patch,
        proposal::{Proposal, ProposalError},
    },
    spec::Spec,
    version::Version,
};

#[derive(Clone, Copy)]
pub enum DraftVisibility {
    Public,
    Private,
    Hidden,
}

pub struct Draft {
    id: Uuid,

    /// The title of the spec.
    title: String,
    /// A short, unique abbreviation for the spec.
    abbreviation: String,

    visibility: DraftVisibility,

    /// The already cut versions of the spec. The latest version is the last element in the vector.
    versions: Vec<Version>,

    /// The patches that will be applied to the next version of the spec when cut.
    unversioned_patches: Vec<Patch>,

    /// The proposals for this draft. If accepted, the proposals will be converted to patches and added to the unversioned_patches.
    proposals: Vec<Proposal>,
}

impl Draft {
    pub fn new(title: String, abbreviation: String, visibility: DraftVisibility) -> Draft {
        Draft {
            id: Uuid::new_v4(),
            title,
            abbreviation,
            visibility,
            versions: Vec::new(),
            unversioned_patches: Vec::new(),
            proposals: Vec::new(),
        }
    }
}

pub enum DraftPublishError {
    /// The draft has unversioned patches that need to be applied before finalizing the draft.
    UnversionedPatchesNotEmpty,
}

impl Draft {
    /// Finalizes the draft and returns the finalized spec.
    ///
    /// The unversioned_patches need to be empty before finalizing the draft.
    pub fn publish(self) -> Result<Spec, DraftPublishError> {
        if !self.unversioned_patches.is_empty() {
            return Err(DraftPublishError::UnversionedPatchesNotEmpty);
        }

        Ok(Spec {
            id: self.id,
            title: self.title,
            abbreviation: self.abbreviation,
            versions: self.versions,
        })
    }
}

impl Draft {
    /// Adds a proposal to the draft and returns the proposal ID.
    pub fn add_proposal(&mut self, proposed_by: Uuid, content_uri: String) -> Uuid {
        let proposal = Proposal::new(proposed_by, content_uri);
        let proposal_id = proposal.id().clone();
        self.proposals.push(proposal);
        proposal_id
    }
}

pub enum ProposalAcceptanceError {
    ProposalNotFound,
    ProposalNotReady,
}

impl Draft {
    pub fn accept_proposal(
        &mut self,
        proposal_id: Uuid,
        accepted_by: Uuid,
    ) -> Result<(), ProposalAcceptanceError> {
        let proposal_index = self
            .proposals
            .iter()
            .position(|p| p.id() == &proposal_id)
            .ok_or(ProposalAcceptanceError::ProposalNotFound)?;

        let Some(proposal) = self.proposals.get(proposal_index) else {
            return Err(ProposalAcceptanceError::ProposalNotFound);
        };

        if !proposal.is_ready() {
            return Err(ProposalAcceptanceError::ProposalNotReady);
        }

        let proposal = self.proposals.remove(proposal_index);

        let patch = proposal.into_patch(accepted_by).map_err(|e| match e {
            ProposalError::ProposalNotReady => ProposalAcceptanceError::ProposalNotReady,
        })?;

        self.unversioned_patches.push(patch);

        Ok(())
    }
}

pub enum CutVersionError {
    NoUnversionedPatches,
}

impl Draft {
    pub fn cut_version(&mut self) -> Result<(), CutVersionError> {
        if self.unversioned_patches.is_empty() {
            return Err(CutVersionError::NoUnversionedPatches);
        }

        let patches = std::mem::take(&mut self.unversioned_patches);
        let version = Version::new(patches);
        self.versions.push(version);
        Ok(())
    }
}

impl Draft {
    pub fn reconstitute(
        id: Uuid,
        title: String,
        abbreviation: String,
        visibility: DraftVisibility,
        versions: Vec<Version>,
        unversioned_patches: Vec<Patch>,
        proposals: Vec<Proposal>,
    ) -> Draft {
        Draft {
            id,
            title,
            abbreviation,
            visibility,
            versions,
            unversioned_patches,
            proposals,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn abbreviation(&self) -> &str {
        &self.abbreviation
    }

    pub fn visibility(&self) -> &DraftVisibility {
        &self.visibility
    }

    pub fn versions(&self) -> &[Version] {
        &self.versions
    }

    pub fn unversioned_patches(&self) -> &[Patch] {
        &self.unversioned_patches
    }

    pub fn proposals(&self) -> &[Proposal] {
        &self.proposals
    }
}
