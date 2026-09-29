use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    application::contract::{RepositoryError, spec_repository::SpecRepository},
    domain::entity::spec::Spec,
};

/// Keeps specs in memory. Everything is lost when the process stops.
#[derive(Default)]
pub struct InMemorySpecRepository {
    specs: RwLock<HashMap<Uuid, Spec>>,
}

#[async_trait]
impl SpecRepository for InMemorySpecRepository {
    async fn register(&self, spec: &Spec) -> Result<(), RepositoryError> {
        let mut specs = self.specs.write().await;
        if specs.contains_key(spec.id()) {
            return Err(RepositoryError::AlreadyExists);
        }
        specs.insert(*spec.id(), spec.clone());
        Ok(())
    }

    async fn save(&self, spec: &Spec) -> Result<(), RepositoryError> {
        let mut specs = self.specs.write().await;
        let Some(stored) = specs.get_mut(spec.id()) else {
            return Err(RepositoryError::NotFound);
        };
        *stored = spec.clone();
        Ok(())
    }

    async fn find_by_id(&self, spec_id: Uuid) -> Result<Option<Spec>, RepositoryError> {
        Ok(self.specs.read().await.get(&spec_id).cloned())
    }

    async fn delete_by_id(&self, spec_id: Uuid) -> Result<(), RepositoryError> {
        match self.specs.write().await.remove(&spec_id) {
            Some(_) => Ok(()),
            None => Err(RepositoryError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_spec(id: Uuid, title: &str) -> Spec {
        Spec::reconstitute(
            id,
            title.to_string(),
            "ABBR".to_string(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    #[tokio::test]
    async fn register_then_find_returns_the_spec() {
        let repository = InMemorySpecRepository::default();
        let id = Uuid::new_v4();

        repository.register(&make_spec(id, "Title")).await.unwrap();

        let found = repository.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(found.title(), "Title");
    }

    #[tokio::test]
    async fn register_fails_when_the_id_already_exists() {
        let repository = InMemorySpecRepository::default();
        let id = Uuid::new_v4();
        repository.register(&make_spec(id, "Title")).await.unwrap();

        let result = repository.register(&make_spec(id, "Other")).await;

        assert_eq!(result, Err(RepositoryError::AlreadyExists));
    }

    #[tokio::test]
    async fn save_replaces_an_existing_spec() {
        let repository = InMemorySpecRepository::default();
        let id = Uuid::new_v4();
        repository.register(&make_spec(id, "Title")).await.unwrap();

        repository.save(&make_spec(id, "Updated")).await.unwrap();

        let found = repository.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(found.title(), "Updated");
    }

    #[tokio::test]
    async fn save_fails_when_the_spec_does_not_exist() {
        let repository = InMemorySpecRepository::default();

        let result = repository.save(&make_spec(Uuid::new_v4(), "Title")).await;

        assert_eq!(result, Err(RepositoryError::NotFound));
    }

    #[tokio::test]
    async fn delete_removes_the_spec() {
        let repository = InMemorySpecRepository::default();
        let id = Uuid::new_v4();
        repository.register(&make_spec(id, "Title")).await.unwrap();

        repository.delete_by_id(id).await.unwrap();

        assert!(repository.find_by_id(id).await.unwrap().is_none());
        assert_eq!(
            repository.delete_by_id(id).await,
            Err(RepositoryError::NotFound)
        );
    }
}
