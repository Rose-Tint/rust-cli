use proc_macro2::TokenStream;
use quote::quote;
use syn::{Result, *};

use crate::err::syn_err;
use crate::utils::*;

mod flag;
mod attrs;
use attrs::*;
mod field;
use field::*;


// NEEDS TO:
// - for each field, create a function to set it to the appropriate value
// - use pattern matching to match against each option


pub fn derive_cmdline(input: &DeriveInput) -> Result<TokenStream> {
    use Data::Struct;
    use Fields::Named;
    if let Struct(DataStruct{fields: Named(fields), .. }) = &input.data {
        let fields = CmdLineFields::from_fields(fields)?;
        // println!("[DEBUG] BEGIN printing flag fields:");
        // for field in &fields.flag_fields {
        //     println!("[DEBUG] {field:?}");
        // }
        // println!("[DEBUG] END printing flag fields:");

        let impl_header = impl_header(&input, "CmdLine");
        let command_type: Type = fields.command_field.get().cloned()
            .map_or(parse_quote! { () }, |c| c.ty);
        let flags_iter = new_var("args_iter");
        let cmd_iter = new_var("command_iter");
        let args_vec = new_var("args_vec");
        let is_valid_flag = fields.make_flag_predicate();
        let command_init = fields.command_field
            .map(|field| field.make_init(&cmd_iter));
        let flag_inits = fields.flag_fields.iter()
            .map(FlagField::make_init);
        let flag_var = new_var("flag_var");
        let flag_args = new_var("flag_args");
        let flag_matcher = fields.make_flag_matcher(&flag_var, &flag_args)?;
        let constructor = fields.make_constructor();
        let internal_path = internal_path();

        return Ok(quote! {
            #impl_header {
                type CommandType = #command_type;
                fn parse(#args_vec: Vec<String>) -> err::Result<Self> {
                    use #internal_path::*;
                    let CommandLine {
                        flag_iter: #flags_iter,
                        command_iter: #cmd_iter,
                    } = CommandLine::from_iter(#args_vec.into_iter(), #is_valid_flag);
                    #command_init;
                    #(#flag_inits;)*
                    for (#flag_var, #flag_args) in #flags_iter {
                        #flag_matcher
                    }
                    return Ok(#constructor);
                }
            }
        });
    }
    return syn_err(input, "`#[derive(CmdLine)]` only supports structs with named fields");
}

pub (super) struct CmdLineFields {
    command_field: SetOnce<CommandField>,
    flag_fields: Vec<FlagField>,
}

impl CmdLineFields {
    fn make_constructor(&self) -> ExprStruct {
        let command = self.command_field
            .map(CommandField::make_constructor_setter)
            .iter();
        // let command = if let Some(cmd) = command.get()
        //     { quote! { #cmd, } } else { quote! {} };
        let flags = self.flag_fields.iter()
            .map(FlagField::make_constructor_setter);
        parse_quote! {
            Self {
                #( #command, )*
                #( #flags, )*
            }
        }
    }

    fn make_flag_matcher(&self, flag_var: &Ident, flag_args: &Ident) -> Result<ExprMatch> {
        let arms: Vec<Arm> = self.flag_fields.iter()
            .map(|f| f.make_match_arm(&flag_var, &flag_args))
            .try_collect()?;
        Ok(parse_quote! {
            match #flag_var.as_str() {
                #( #arms )*
                _ => return Err(UnrecognizedFlag(#flag_var)),
            }
        })
    }

    fn make_flag_predicate(&self) -> ExprClosure {
        let mut patterns = Vec::new();
        for FlagField { ref mods, .. } in &self.flag_fields {
            for name in &mods.names {
                patterns.push(name.to_pattern());
            }
        }
        let patterns = patterns.iter();
        let string_var = new_var("string");
        parse_quote! {
            |#string_var| match #string_var.as_str() {
                #(| #patterns)* => true,
                _ => false,
            }
        }
    }

    fn from_fields(fields: &FieldsNamed) -> Result<Self> {
        use FieldAttr::*;
        let mut command_field = SetOnce::new();
        let mut flag_fields = Vec::new();
        for field in &fields.named {
            let ident = match field.ident {
                Some(ref ident) => ident.clone(),
                None => return syn_err(field, "Named field required"),
            };
            let ty = &field.ty;
            for attr in &field.attrs {
                if let Some(field_attr) = parse_attr(attr.clone())? {
                    match field_attr {
                        Command => {
                            let command = CommandField::new(ident.clone(), ty.clone());
                            command_field.syn_set(command, field, "Too many `#[command]`s")?;
                        },
                        Flag(mods)
                            => flag_fields.push(FlagField::new(ident.clone(), ty.clone(), mods)),
                    };
                }
            }
        }
        return Ok(CmdLineFields { command_field, flag_fields, });
    }
}
