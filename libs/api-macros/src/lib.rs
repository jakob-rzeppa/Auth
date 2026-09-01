#![allow(non_snake_case)]

//! Procedural macros for building HTTP API types.

mod api_error_response;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Error, ItemEnum};

/// Generates an [`axum::response::IntoResponse`] implementation for an error enum.
///
/// Every variant must carry all three of `#[status_code(..)]`, `#[error(..)]` and
/// `#[description(..)]` - there are no defaults. The error code and description are
/// format strings in the style of `thiserror`, so they may interpolate the variant's
/// own fields: named fields by name (`{email}`), tuple fields by position (`{0}`).
///
/// ```ignore
/// #[ApiErrorResponse]
/// pub enum CreateUserErrorResponse {
///     #[status_code(axum::http::StatusCode::BAD_REQUEST)]
///     #[error("invalid_request")]
///     #[description("Invalid request body.")]
///     InvalidBody,
/// }
/// ```
///
/// The response body is `{"error": <code>, "error_description": <description>}`.
#[proc_macro_attribute]
pub fn ApiErrorResponse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = TokenStream2::from(attr);
    if !attr.is_empty() {
        return Error::new_spanned(attr, "`ApiErrorResponse` does not take any arguments")
            .to_compile_error()
            .into();
    }

    let item = syn::parse_macro_input!(item as ItemEnum);

    match api_error_response::expand(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
