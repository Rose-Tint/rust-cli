use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils::internal_path;

pub fn cmdline(input: &DeriveInput) -> TokenStream {
    let DeriveInput { ident, generics, .. } = &input;
    let (impl_tps, typars, where_clause) = generics.split_for_impl();
    let internal_path = internal_path();
    quote! {
        #[automatically_derived]
        impl #impl_tps #internal_path::CmdLine for #ident #typars #where_clause {
            type CommandType = ();
            fn parse(_: Vec<String>) -> err::Result<Self> {
                todo!()
            }
        }
    }
}
