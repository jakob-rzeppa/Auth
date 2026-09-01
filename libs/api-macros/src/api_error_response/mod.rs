//! Expansion of the `#[ApiErrorResponse]` attribute macro.

mod attributes;
mod variant;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemEnum};

/// Re-emits the enum without its helper attributes and appends the generated
/// `IntoResponse` implementation.
pub fn expand(mut item: ItemEnum) -> Result<TokenStream, Error> {
    let mut errors: Option<Error> = None;
    let mut arms = Vec::with_capacity(item.variants.len());

    for variant in &item.variants {
        match variant::arm_for(variant) {
            Ok(arm) => arms.push(arm),
            Err(err) => combine(&mut errors, err),
        }
    }

    if let Some(err) = errors {
        return Err(err);
    }

    // An attribute macro replaces the item it is applied to, so the enum has to be
    // emitted again - without the helper attributes, which no longer exist by then.
    for variant in &mut item.variants {
        variant.attrs.retain(|attr| !attributes::is_helper(attr));
    }

    let ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    Ok(quote! {
        #item

        impl #impl_generics axum::response::IntoResponse for #ident #ty_generics #where_clause {
            fn into_response(self) -> axum::response::Response {
                let (status, error_code, error_description): (
                    axum::http::StatusCode,
                    ::std::string::String,
                    ::std::string::String,
                ) = match self {
                    #(#arms)*
                };

                let body = axum::Json(::serde_json::json!({
                    "error": error_code,
                    "error_description": error_description,
                }));
                axum::response::IntoResponse::into_response((status, body))
            }
        }
    })
}

/// Accumulates `err` so that every problem in the enum is reported at once, rather
/// than only the first one.
fn combine(errors: &mut Option<Error>, err: Error) {
    match errors {
        Some(existing) => existing.combine(err),
        None => *errors = Some(err),
    }
}
