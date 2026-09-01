#![allow(non_snake_case)]

//! Procedural macros for building HTTP API types.

mod api_error_response;
mod api_request;
mod api_response;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Error, Expr, ItemEnum, ItemStruct};

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

/// Generates an [`axum::response::IntoResponse`] implementation for a success response
/// struct and derives [`serde::Serialize`] on it.
///
/// The macro takes a single argument: the [`axum::http::StatusCode`] expression the
/// response should carry. The struct is serialised into the JSON body.
///
/// ```ignore
/// #[ApiResponse(axum::http::StatusCode::CREATED)]
/// pub struct CreateUserResponse {
///     pub id: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn ApiResponse(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        return Error::new(
            proc_macro2::Span::call_site(),
            "`ApiResponse` takes a status code expression, e.g. `#[ApiResponse(axum::http::StatusCode::CREATED)]`",
        )
        .to_compile_error()
        .into();
    }

    let status = syn::parse_macro_input!(attr as Expr);
    let item = syn::parse_macro_input!(item as ItemStruct);

    match api_response::expand(status, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Generates an [`axum::extract::FromRequest`] implementation for a request struct
/// and derives [`serde::Deserialize`] on it.
///
/// The implementation reads the entire request body and deserialises it from JSON.
/// Any failure - reading the body or deserialising it - is turned into the error
/// given as the macro's single argument, which must be an enum-variant path. The
/// enum it names becomes the rejection type.
///
/// ```ignore
/// #[ApiRequest(UpdateUserErrorResponse::InvalidBody)]
/// pub struct UpdateUserRequest {
///     pub email: Option<String>,
/// }
/// ```
#[proc_macro_attribute]
pub fn ApiRequest(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        return Error::new(
            proc_macro2::Span::call_site(),
            "`ApiRequest` takes an enum-variant path, e.g. `#[ApiRequest(UpdateUserErrorResponse::InvalidBody)]`",
        )
        .to_compile_error()
        .into();
    }

    let error = syn::parse_macro_input!(attr as Expr);
    let item = syn::parse_macro_input!(item as ItemStruct);

    match api_request::expand(error, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
