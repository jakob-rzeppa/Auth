use axum::extract::{FromRequest, Request};

use crate::api::privileges::error::PrivilegeApiError;

pub struct CreatePrivilegeRequest {
    name: String,
    description: String,
}

impl<S: Send + Sync> FromRequest<S> for CreatePrivilegeRequest {
    type Rejection = PrivilegeApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}

pub struct PatchPrivilegeRequest {
    name: Option<String>,
    description: Option<String>,
}

impl<S: Send + Sync> FromRequest<S> for PatchPrivilegeRequest {
    type Rejection = PrivilegeApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}
