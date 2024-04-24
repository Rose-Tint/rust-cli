use syn::spanned::Spanned;


pub fn syn_err<T>(input: impl Spanned, msg: &str) -> syn::Result<T> {
    Err(syn::Error::new(input.span(), msg))
}
