//! Expansion of the `#[ApiResponse(..)]` attribute macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Expr, ItemStruct, Token};

use crate::headers::Headers;

/// The macro's arguments: a status code expression, optionally followed by
/// `headers(..)`.
pub struct ApiResponseArgs {
    status: Expr,
    headers: Headers,
}

impl Parse for ApiResponseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let status: Expr = input.parse()?;

        let mut headers = Headers::default();
        if input.parse::<Option<Token![,]>>()?.is_some() && !input.is_empty() {
            headers = input.parse()?;
            input.parse::<Option<Token![,]>>()?;
        }

        Ok(Self { status, headers })
    }
}

/// Re-emits the struct with a `#[derive(Serialize)]` and appends an `IntoResponse`
/// implementation that answers with the status, the configured headers and the struct
/// serialised as JSON.
pub fn expand(args: ApiResponseArgs, item: ItemStruct) -> Result<TokenStream, Error> {
    let ApiResponseArgs { status, headers } = args;
    let ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let response = match headers.to_tokens() {
        Some(headers) => quote! { (status, #headers, axum::Json(self)) },
        None => quote! { (status, axum::Json(self)) },
    };

    Ok(quote! {
        #[derive(::serde::Serialize)]
        #item

        impl #impl_generics axum::response::IntoResponse for #ident #ty_generics #where_clause {
            fn into_response(self) -> axum::response::Response {
                let status: axum::http::StatusCode = #status;
                axum::response::IntoResponse::into_response(#response)
            }
        }
    })
}
