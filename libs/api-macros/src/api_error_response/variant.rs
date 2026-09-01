//! Building the `match` arm that a single enum variant expands into.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Expr, Fields, Variant};

use super::attributes::{format_string, required};

/// Turns one variant into the arm producing its status, error code and description.
pub fn arm_for(variant: &Variant) -> Result<TokenStream, Error> {
    let mut errors: Option<Error> = None;

    let status_code = required(variant, "status_code", &mut errors, |attr| {
        attr.parse_args::<Expr>()
    });
    let error_code = required(variant, "error", &mut errors, format_string);
    let description = required(variant, "description", &mut errors, format_string);

    if let Some(err) = errors {
        return Err(err);
    }

    let (status_code, error_code, description) = (
        status_code.unwrap(),
        error_code.unwrap(),
        description.unwrap(),
    );

    let ident = &variant.ident;
    let (pattern, args) = match &variant.fields {
        Fields::Unit => (quote!(Self::#ident), Vec::new()),
        // Named fields are bound under their own names, so a format string can pick
        // them up through implicit capture: `{email}`.
        Fields::Named(fields) => {
            let names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().expect("named field has an ident"));
            (quote!(Self::#ident { #(#names),* }), Vec::new())
        }
        // Tuple fields are passed positionally instead, so `{0}` works like it does
        // in `thiserror`.
        Fields::Unnamed(fields) => {
            let bindings: Vec<_> = (0..fields.unnamed.len())
                .map(|index| format_ident!("__f{}", index))
                .collect();
            (quote!(Self::#ident( #(#bindings),* )), bindings)
        }
    };

    Ok(quote! {
        #[allow(unused_variables)]
        #pattern => (
            #status_code,
            ::std::format!(#error_code #(, #args)*),
            ::std::format!(#description #(, #args)*),
        ),
    })
}
