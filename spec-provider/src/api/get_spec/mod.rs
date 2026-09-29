use spec_api::{GetSpecRequest, GetSpecResponse, get_spec_response::Kind};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    api::SpecProviderService,
    application::spec_service::get::{GetSpecError, SpecOrDraftProjection},
};

impl SpecProviderService {
    pub(super) async fn get_spec_endpoint(
        &self,
        request: Request<GetSpecRequest>,
    ) -> Result<Response<GetSpecResponse>, Status> {
        let Ok(id) = Uuid::parse_str(&request.into_inner().id) else {
            return Err(Status::invalid_argument("The provided spec ID is invalid."));
        };

        let projection = self
            .spec_service
            .get_spec_or_draft(id)
            .await
            .map_err(|e| match e {
                GetSpecError::SpecNotFound => Status::not_found("The spec was not found."),
                GetSpecError::DatabaseError => {
                    Status::internal("An internal server error occurred.")
                }
            })?;

        let kind = match projection {
            SpecOrDraftProjection::Spec(spec) => Kind::Published(spec.into()),
            SpecOrDraftProjection::Draft(draft) => Kind::Draft(draft.into()),
        };

        Ok(Response::new(GetSpecResponse { kind: Some(kind) }))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tonic::Code;

    use super::*;
    use crate::{
        application::{contract::spec_repository::SpecRepository, spec_service::SpecService},
        domain::entity::spec::Spec,
        persistence::in_memory::{
            draft_repository::InMemoryDraftRepository, spec_repository::InMemorySpecRepository,
        },
    };

    fn make_service(specs: Arc<InMemorySpecRepository>) -> SpecProviderService {
        SpecProviderService::new(SpecService::new(
            specs,
            Arc::new(InMemoryDraftRepository::default()),
        ))
    }

    fn request(id: &str) -> Request<GetSpecRequest> {
        Request::new(GetSpecRequest { id: id.to_string() })
    }

    #[tokio::test]
    async fn returns_the_published_spec() {
        let id = Uuid::new_v4();
        let specs = Arc::new(InMemorySpecRepository::default());
        specs
            .register(&Spec::reconstitute(
                id,
                "Spec".to_string(),
                "SPEC".to_string(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ))
            .await
            .unwrap();

        let response = make_service(specs)
            .get_spec_endpoint(request(&id.to_string()))
            .await
            .unwrap()
            .into_inner();

        let Some(Kind::Published(spec)) = response.kind else {
            panic!("expected a published spec");
        };
        assert_eq!(spec.id, id.to_string());
    }

    #[tokio::test]
    async fn fails_with_invalid_argument_for_a_malformed_id() {
        let service = make_service(Arc::new(InMemorySpecRepository::default()));

        let status = service
            .get_spec_endpoint(request("not-a-uuid"))
            .await
            .unwrap_err();

        assert_eq!(status.code(), Code::InvalidArgument);
    }

    #[tokio::test]
    async fn fails_with_not_found_for_an_unknown_id() {
        let service = make_service(Arc::new(InMemorySpecRepository::default()));

        let status = service
            .get_spec_endpoint(request(&Uuid::new_v4().to_string()))
            .await
            .unwrap_err();

        assert_eq!(status.code(), Code::NotFound);
    }
}
