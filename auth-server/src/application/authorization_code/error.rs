use crate::domain::entity::authorization_code::request::validate::{
    FatalValidationError, RedirectableValidationError, ValidationError,
};

#[derive(Debug, PartialEq)]
pub enum FatalAuthCodeError {
    ClientNotFound,
    ClientIdMismatch,
    InvalidRedirectUri,
    InvalidState,
    DatabaseError,
    PushedRequestNotFound,
}

#[derive(Debug, PartialEq)]
pub enum RedirectableAuthCodeError {
    InvalidResponseType,
    InvalidScope,
    InvalidCodeChallengeMethod,
    InvalidCodeChallenge,
    DatabaseError,
}

#[derive(Debug, PartialEq)]
pub enum AuthCodeError {
    Fatal {
        error: FatalAuthCodeError,
    },
    Redirectable {
        error: RedirectableAuthCodeError,
        redirect_uri: String,
        state: String,
    },
}

impl AuthCodeError {
    pub fn fatal(error: FatalAuthCodeError) -> Self {
        AuthCodeError::Fatal { error }
    }

    pub fn redirectable(
        error: RedirectableAuthCodeError,
        redirect_uri: String,
        state: String,
    ) -> Self {
        AuthCodeError::Redirectable {
            error,
            redirect_uri,
            state,
        }
    }
}

impl From<ValidationError> for AuthCodeError {
    fn from(validation_error: ValidationError) -> Self {
        match validation_error {
            ValidationError::Fatal { error } => AuthCodeError::Fatal {
                error: match error {
                    FatalValidationError::ClientNotFound => FatalAuthCodeError::ClientNotFound,
                    FatalValidationError::ClientIdMismatch => FatalAuthCodeError::ClientIdMismatch,
                    FatalValidationError::InvalidRedirectUri => {
                        FatalAuthCodeError::InvalidRedirectUri
                    }
                    FatalValidationError::InvalidState => FatalAuthCodeError::InvalidState,
                },
            },
            ValidationError::Redirectable {
                error,
                redirect_uri,
                state,
            } => AuthCodeError::Redirectable {
                error: match error {
                    RedirectableValidationError::InvalidResponseType => {
                        RedirectableAuthCodeError::InvalidResponseType
                    }
                    RedirectableValidationError::InvalidScope => {
                        RedirectableAuthCodeError::InvalidScope
                    }
                    RedirectableValidationError::InvalidCodeChallengeMethod => {
                        RedirectableAuthCodeError::InvalidCodeChallengeMethod
                    }
                    RedirectableValidationError::InvalidCodeChallenge => {
                        RedirectableAuthCodeError::InvalidCodeChallenge
                    }
                },
                redirect_uri,
                state,
            },
        }
    }
}
