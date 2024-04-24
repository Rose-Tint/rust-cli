pub use proc_macro2::Span;
use syn::spanned as syn;


/// A custom implementation to get around the `ToTokens` bound on
/// `syn`'s `Spanned`
pub trait Spanned {
    fn span(&self) -> Span;
}

pub fn join_spans(s1: impl Spanned, s2: Option<impl Spanned>) -> Span {
    let span = s1.span();
    if let Some(val) = s2 {
        if let Some(new) = span.join(val.span()) {
            return new;
        }
    }
    return span;
}

impl<T: syn::Spanned> Spanned for T {
    fn span(&self) -> Span {
        self.span()
    }
}
