use quote::*;
use proc_macro2::{Ident, TokenStream};
use syn::*;

#[allow(unused_imports)]
pub use cli_lexer::internal::flag_init;

mod set_once;
mod span;
pub use set_once::*;
#[allow(unused_imports)]
pub use span::*;


pub (crate) fn new_var(name: &str) -> Ident {
    format_ident!("__cli_{name}")
}

pub (crate) fn new_from(ident: &Ident) -> Ident {
    format_ident!("__cli_{ident}")
}

#[macro_export]
macro_rules! internal {
    () => {
        (parse_quote! { ::cli::internal } as Path)
    };
    ($path:path) => {
        (parse_quote! { ::cli::internal::$path } as Path)
    };
    ($name:literal) => {{
        let ident: Ident = Ident::new(name, Span::call_site());
        (parse_quote! { ::cli::internal::#ident } as Path)
    }};
}

pub fn internal_path() -> Path {
    parse_quote! {
        ::cli::internal
    }
}

pub (crate) fn impl_header(input: &DeriveInput, trait_name: &str) -> TokenStream {
    let DeriveInput { ident, generics, .. } = &input;
    let (impl_tps, typars, where_clause)
        = generics.split_for_impl();
    let trait_name = format_ident!("{trait_name}");
    let internal_path = internal_path();
    quote! {
        #[automatically_derived]
        impl #impl_tps #internal_path::#trait_name for #ident #typars #where_clause
    }
}
