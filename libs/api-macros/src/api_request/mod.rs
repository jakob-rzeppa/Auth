//! Expansion of the `#[ApiRequest(..)]` attribute macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Expr, ItemStruct, spanned::Spanned};

/// Re-emits the struct with `#[derive(Deserialize)]` and appends a `FromRequest`
/// implementation that reads the whole body and deserialises it from JSON, turning
/// any failure into `error`.
///
/// `error` must be an enum-variant path such as `UpdateUserErrorResponse::InvalidBody`;
/// the enum it names (`UpdateUserErrorResponse`) becomes the rejection type.
pub fn expand(error: Expr, item: ItemStruct) -> Result<TokenStream, Error> {
    if !item.generics.params.is_empty() {
        return Err(Error::new(
            item.generics.span(),
            "`ApiRequest` does not support generic structs",
        ));
    }

    let rejection = rejection_type(&error)?;
    let ident = &item.ident;

    Ok(quote! {
        #[derive(::serde::Deserialize)]
        #item

        impl<S: ::std::marker::Send + ::std::marker::Sync> axum::extract::FromRequest<S> for #ident {
            type Rejection = #rejection;

            async fn from_request(
                req: axum::extract::Request,
                state: &S,
            ) -> ::std::result::Result<Self, Self::Rejection> {
                let bytes = <axum::body::Bytes as axum::extract::FromRequest<S>>::from_request(req, state)
                    .await
                    .map_err(|_| #error)?;

                ::serde_json::from_slice(&bytes).map_err(|_| #error)
            }
        }
    })
}

/// Derives the rejection type from the error expression by dropping the last segment
/// of its path: `UpdateUserErrorResponse::InvalidBody` -> `UpdateUserErrorResponse`.
fn rejection_type(error: &Expr) -> Result<syn::Path, Error> {
    let Expr::Path(expr) = error else {
        return Err(Error::new(
            error.span(),
            "`ApiRequest` takes an enum-variant path, e.g. \
             `#[ApiRequest(UpdateUserErrorResponse::InvalidBody)]`",
        ));
    };

    let mut path = expr.path.clone();
    if path.segments.len() < 2 {
        return Err(Error::new(
            error.span(),
            "`ApiRequest` takes an enum-variant path, e.g. \
             `#[ApiRequest(UpdateUserErrorResponse::InvalidBody)]`",
        ));
    }

    path.segments.pop();
    // `pop` leaves a trailing `::`; rebuild without it.
    let segments = path.segments.into_iter().collect::<syn::punctuated::Punctuated<_, _>>();
    Ok(syn::Path {
        leading_colon: expr.path.leading_colon,
        segments,
    })
}
