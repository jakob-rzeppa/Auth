//! Reading the `#[status_code(..)]`, `#[error(..)]` and `#[description(..)]`
//! helper attributes off a variant.

use syn::{Attribute, Error, LitStr, Variant, spanned::Spanned};

use super::combine;

const HELPERS: [&str; 3] = ["status_code", "error", "description"];

/// Reads the single occurrence of `name` on `variant`, recording a problem in
/// `errors` if it is missing or repeated.
pub fn required<T>(
    variant: &Variant,
    name: &str,
    errors: &mut Option<Error>,
    parse: impl Fn(&Attribute) -> Result<T, Error>,
) -> Option<T> {
    let mut found = None;
    let mut seen = false;

    for attr in variant
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(name))
    {
        if seen {
            combine(
                errors,
                Error::new_spanned(attr, format!("duplicate `#[{name}(..)]` attribute")),
            );
            continue;
        }
        seen = true;

        match parse(attr) {
            Ok(value) => found = Some(value),
            // The attribute is there but unusable; the parse error already says so,
            // so don't also report it as missing.
            Err(err) => combine(errors, err),
        }
    }

    if !seen {
        combine(
            errors,
            Error::new(
                variant.ident.span(),
                format!(
                    "missing `#[{name}(..)]` attribute on variant `{}`",
                    variant.ident
                ),
            ),
        );
    }

    found
}

/// Parses an attribute whose body must be exactly one format string.
pub fn format_string(attr: &Attribute) -> Result<LitStr, Error> {
    attr.parse_args::<LitStr>().map_err(|_| {
        Error::new(
            attr.span(),
            format!(
                "`#[{}(..)]` takes exactly one string literal, which may interpolate \
                 the variant's fields (`{{name}}` for named fields, `{{0}}` for tuple fields)",
                attr.path()
                    .get_ident()
                    .map(ToString::to_string)
                    .unwrap_or_default()
            ),
        )
    })
}

/// Whether `attr` is one of the helpers this macro consumes, and so must be stripped
/// from the re-emitted enum.
pub fn is_helper(attr: &Attribute) -> bool {
    HELPERS.iter().any(|name| attr.path().is_ident(name))
}
