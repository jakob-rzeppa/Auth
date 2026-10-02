//! Parsing of the optional `headers(name => value, ..)` macro argument, shared by
//! `#[ApiResponse]` and `#[ApiErrorResponse]`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token, parenthesized};

/// The `(name, value)` expression pairs of a `headers(..)` argument. Empty when the
/// argument was not given.
#[derive(Default)]
pub struct Headers {
    entries: Vec<(Expr, Expr)>,
}

impl Headers {
    /// The headers as an array of `(name, value)` tuples, usable as a response part.
    /// Nothing when there are no headers: an empty array has no inferable type.
    pub fn to_tokens(&self) -> Option<TokenStream> {
        if self.entries.is_empty() {
            return None;
        }

        let pairs = self
            .entries
            .iter()
            .map(|(name, value)| quote! { (#name, #value) });
        Some(quote! { [#(#pairs),*] })
    }
}

impl Parse for Headers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keyword: Ident = input.parse()?;
        if keyword != "headers" {
            return Err(syn::Error::new(keyword.span(), "expected `headers(..)`"));
        }

        let content;
        parenthesized!(content in input);
        let entries = content.parse_terminated(
            |entry: ParseStream| {
                let name: Expr = entry.parse()?;
                entry.parse::<Token![=>]>()?;
                let value: Expr = entry.parse()?;
                Ok((name, value))
            },
            Token![,],
        )?;

        Ok(Self {
            entries: entries.into_iter().collect(),
        })
    }
}
