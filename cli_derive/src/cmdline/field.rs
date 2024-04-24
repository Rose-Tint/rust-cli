#![macro_use]

use syn::*;
use crate::utils::*;

use super::flag::*;


#[derive(Debug)]
pub (super) struct FlagField {
    pub ident: Ident,
    pub ty: Type,
    var_name: Ident,
    pub mods: FlagMods,
    span: Span,
}

#[derive(Debug, Clone)]
pub (super) struct CommandField {
    pub ident: Ident,
    pub ty: Type,
    pub (super) var_name: Ident,
    span: Span,
    // descr: Option<LitStr>,
}

impl FlagField {
    pub (super) fn new(ident: Ident, ty: Type, mods: FlagMods) -> Self {
        let span = join_spans(&ident, Some(&ty));
        let mut var_name = new_from(&ident);
        var_name.set_span(span);
        FlagField { ident, ty, var_name, mods, span }
    }

    pub (super) fn make_match_arm(&self, flag_var: &Ident, flag_args: &Ident) -> Result<Arm> {
        let pat = self.mods.get_pattern();
        let arg = self.mods.get_args_case(&self.var_name, flag_var, &flag_args)?;
        let noarg = self.mods.get_noarg_case(&self.var_name, flag_var)?;
        Ok(parse_quote! {
            #pat => {
                if #flag_args.is_empty() {
                    #noarg
                } else {
                    #arg
                }
            },
        })
    }

    pub (super) fn make_init(&self) -> Stmt {
        let init_var = &self.var_name;
        let ident = &self.ident;
        let ty = &self.ty;
        // let internal_path = internal_path();
        let value: Expr = if let Some(default) = &self.mods.default.get() {
            parse_quote_spanned! {default.span()=>
                FlagInit::default(#default.into())
            }
        } else {
            parse_quote_spanned! {self.span=>
                FlagInit::required(stringify!(#ident))
            }
        };
        parse_quote! {
            let mut #init_var: FlagInit<#ty> = #value;
        }
    }

    pub (super) fn make_constructor_setter(&self) -> FieldValue {
        let field = &self.ident;
        let var = &self.var_name;
        parse_quote_spanned! {self.span=>
            #field: #var.unwrap()?
        }
    }
}

impl Spanned for FlagField {
    fn span(&self) -> Span {
        self.span
    }
}

impl CommandField {
    pub (super) fn new(ident: Ident, ty: Type) -> Self {
        let span = join_spans(&ident, Some(&ty));
        let mut var_name = new_from(&ident);
        var_name.set_span(span);
        CommandField { ident, ty, var_name, span }
    }

    pub (super) fn make_init(&self, command_iter: &Ident) -> Stmt {
        let CommandField { ref ty, ref var_name, .. } = self;
        parse_quote_spanned! {self.span=>
            let mut #var_name: #ty = <#ty as Command>::parse_command(#command_iter)?;
        }
    }

    pub (super) fn make_constructor_setter(&self) -> FieldValue {
        let field = &self.ident;
        let var = &self.var_name;
        parse_quote_spanned! {self.span=>
            #field: #var
        }
    }
}

impl Spanned for CommandField {
    fn span(&self) -> Span {
        self.span
    }
}

// ########## TESTS ##########

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use quote::ToTokens;

    #[test]
    fn test_flag_make_init() {
        // let internal_path = internal_path();
        let ident = Ident::new("test_ident", Span::call_site());
        let ty: Type = parse_quote! { TestType };
        let mods = FlagMods { ..Default::default() };
        let init_var = new_from(&ident);
        let mut flag = FlagField::new(ident.clone(), ty.clone(), mods);
        let created = flag.make_init();
        println!("{}", created.to_token_stream());
        let expected: Stmt = parse_quote! {
            let mut #init_var: FlagInit<#ty> = FlagInit::required(stringify!(#ident));
        };
        assert_eq!(created, expected, "without a default value");
        let default: Expr = parse_quote!(DefaultValue);
        let _ = flag.mods.default.set(default.clone(), ());
        let created = flag.make_init();
        println!("{}", created.to_token_stream());
        let expected: Stmt = parse_quote! {
            let mut #init_var: FlagInit<#ty> = FlagInit::default(#default.into());
        };
        assert_eq!(created, expected, "with a default value");
    }

    #[test]
    fn test_command_make_init() {
        let ident = Ident::new("test_ident", Span::call_site());
        let ty: Type = parse_quote!(TestType);
        let init_var = new_from(&ident);
        let command_iter = new_var("command_line");
        let command = CommandField::new(ident, ty.clone());
        let created = command.make_init(&command_iter);
        let expected: Stmt = parse_quote! {
            let mut #init_var: #ty = <#ty as Command>::parse_command(#command_iter)?;
        };
        assert_eq!(created, expected);
    }

    #[test]
    fn test_make_match_arm() {
        let ident = Ident::new("test_ident", Span::call_site());
        let ty: Type = parse_quote!(TestType);
        let mods = FlagMods {
            names: names!["test-flag"],
            ..Default::default()
        };
        let init_var = new_from(&ident);
        let mut flag = FlagField::new(ident.clone(), ty.clone(), mods);
        let flag_var = new_var("flag");
        let flag_args = new_var("flag_args");
        let parser_var = new_var("parser");
        // let internal_path = internal_path();
        let created = flag.make_match_arm(&flag_var, &flag_args);
        assert!(created.is_err());
        let value: Expr = parse_quote!(TestValue);
        let _ = flag.mods.value.set(value.clone(), ());
        let created = flag.make_match_arm(&flag_var, &flag_args).unwrap();
        let expected: Arm = parse_quote! {
            | "--test-flag" => {
                if #flag_args.is_empty() {
                    #init_var.set(#value.into())?
                } else {
                    return Err(TooManyFlagArgs(#flag_var, #flag_args))
                }
            },
        };
        println!(" created: {}\nexpected: {}\n", created.to_token_stream(), expected.to_token_stream());
        assert_eq!(created, expected, "with value, and no parser");
        let parser: Expr = parse_quote!(|s| s.into());
        let _ = flag.mods.parser.set(parser.clone(), ());
        flag.mods.value = SetOnce::new();
        let created = flag.make_match_arm(&flag_var, &flag_args).unwrap();
        let expected: Arm = parse_quote! {
            | "--test-flag" => {
                if #flag_args.is_empty() {
                    return Err(MissingFlagArg(#flag_var))
                } else {
                    {
                        let #parser_var = #parser;
                        #init_var.set(#parser_var(#flag_args)?)?
                    }
                }
            },
        };
        println!(" created: {}\nexpected: {}\n", created.to_token_stream(), expected.to_token_stream());
        assert_eq!(created, expected, "with parser, and no value");
        let _ = flag.mods.value.set(value.clone(), ());
        let created = flag.make_match_arm(&flag_var, &flag_args).unwrap();
        let expected: Arm = parse_quote! {
            | "--test-flag" => {
                if #flag_args.is_empty() {
                    #init_var.set(#value.into())?
                } else {
                    {
                        let #parser_var = #parser;
                        #init_var.set(#parser_var(#flag_args)?)?
                    }
                }
            },
        };
        println!(" created: {}\nexpected: {}\n", created.to_token_stream(), expected.to_token_stream());
        assert_eq!(created, expected, "with both parser and value");
    }
}

