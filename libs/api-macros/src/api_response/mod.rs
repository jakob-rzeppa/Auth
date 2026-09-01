//! Expansion of the `#[ApiResponse(..)]` attribute macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Expr, ItemStruct};

/// Re-emits the struct with a `#[derive(Serialize)]` and appends an `IntoResponse`
/// implementation that answers with `status` and the struct serialised as JSON.
pub fn expand(status: Expr, item: ItemStruct) -> Result<TokenStream, Error> {
    let ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    Ok(quote! {
        #[derive(::serde::Serialize)]
        #item

        impl #impl_generics axum::response::IntoResponse for #ident #ty_generics #where_clause {
            fn into_response(self) -> axum::response::Response {
                let status: axum::http::StatusCode = #status;
                axum::response::IntoResponse::into_response((status, axum::Json(self)))
            }
        }
    })
}
