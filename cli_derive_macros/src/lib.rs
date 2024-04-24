#![allow(dead_code)]
// #![allow(unused)]

use cli_derive::*;

use proc_macro::*;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(CmdLine, attributes(command, flag))]
pub fn derive_cmdline(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    cmdline::derive_cmdline(&input)
        .unwrap_or_else(|err| {
        let mut dummy = dummies::cmdline(&input);
        let err = err.into_compile_error();
        dummy.extend(err);
        return dummy;
    }).into()
}

#[proc_macro_derive(Command, attributes(default))]
pub fn derive_command(_input: TokenStream) -> TokenStream {
    return TokenStream::new();
}

#[proc_macro_derive(Flag, attributes(value, default))]
pub fn derive_flag(_input: TokenStream) -> TokenStream {
    return TokenStream::new();
}
