use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    application::contract::{RepositoryError, draft_repository::DraftRepository},
    domain::entity::spec::draft::Draft,
};

/// Keeps drafts in memory. Everything is lost when the process stops.
#[derive(Default)]
pub struct InMemoryDraftRepository {
    drafts: RwLock<HashMap<Uuid, Draft>>,
}

#[async_trait]
impl DraftRepository for InMemoryDraftRepository {
    async fn register(&self, draft: &Draft) -> Result<(), RepositoryError> {
        let mut drafts = self.drafts.write().await;
        if drafts.contains_key(draft.id()) {
            return Err(RepositoryError::AlreadyExists);
        }
        drafts.insert(*draft.id(), draft.clone());
        Ok(())
    }

    async fn save(&self, draft: &Draft) -> Result<(), RepositoryError> {
        let mut drafts = self.drafts.write().await;
        let Some(stored) = drafts.get_mut(draft.id()) else {
            return Err(RepositoryError::NotFound);
        };
        *stored = draft.clone();
        Ok(())
    }

    async fn find_by_id(&self, draft_id: Uuid) -> Result<Option<Draft>, RepositoryError> {
        Ok(self.drafts.read().await.get(&draft_id).cloned())
    }

    async fn delete_by_id(&self, draft_id: Uuid) -> Result<(), RepositoryError> {
        match self.drafts.write().await.remove(&draft_id) {
            Some(_) => Ok(()),
            None => Err(RepositoryError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::spec::draft::DraftVisibility;

    fn make_draft(id: Uuid, title: &str) -> Draft {
        Draft::reconstitute(
            id,
            title.to_string(),
            "ABBR".to_string(),
            DraftVisibility::Public,
            vec![],
            vec![],
            vec![],
        )
    }

    #[tokio::test]
    async fn register_then_find_returns_the_draft() {
        let repository = InMemoryDraftRepository::default();
        let id = Uuid::new_v4();

        repository.register(&make_draft(id, "Title")).await.unwrap();

        let found = repository.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(found.title(), "Title");
    }

    #[tokio::test]
    async fn register_fails_when_the_id_already_exists() {
        let repository = InMemoryDraftRepository::default();
        let id = Uuid::new_v4();
        repository.register(&make_draft(id, "Title")).await.unwrap();

        let result = repository.register(&make_draft(id, "Other")).await;

        assert_eq!(result, Err(RepositoryError::AlreadyExists));
    }

    #[tokio::test]
    async fn save_replaces_an_existing_draft() {
        let repository = InMemoryDraftRepository::default();
        let id = Uuid::new_v4();
        repository.register(&make_draft(id, "Title")).await.unwrap();

        repository.save(&make_draft(id, "Updated")).await.unwrap();

        let found = repository.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(found.title(), "Updated");
    }

    #[tokio::test]
    async fn save_fails_when_the_draft_does_not_exist() {
        let repository = InMemoryDraftRepository::default();

        let result = repository.save(&make_draft(Uuid::new_v4(), "Title")).await;

        assert_eq!(result, Err(RepositoryError::NotFound));
    }

    #[tokio::test]
    async fn delete_removes_the_draft() {
        let repository = InMemoryDraftRepository::default();
        let id = Uuid::new_v4();
        repository.register(&make_draft(id, "Title")).await.unwrap();

        repository.delete_by_id(id).await.unwrap();

        assert!(repository.find_by_id(id).await.unwrap().is_none());
        assert_eq!(
            repository.delete_by_id(id).await,
            Err(RepositoryError::NotFound)
        );
    }
}
