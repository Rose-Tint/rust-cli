#![macro_use]

use cli_lexer::Raw;
use proc_macro2::Span;
use syn::{Result, *};
use syn::punctuated::Punctuated;
use syn::parse::*;

use crate::err::syn_err;
use crate::utils::*;


#[derive(Default, Debug, PartialEq)]
pub (super) struct FlagMods {
    pub (super) names: Vec<FlagName>,
    pub (super) default: SetOnce<Expr>,
    pub (super) value: SetOnce<Expr>,
    pub (super) parser: SetOnce<Expr>,
}

impl FlagMods {
    /// creates the `syn::Pat` for matching against
    pub (super) fn get_pattern(&self) -> Pat {
        let mut cases = Vec::new();
        for name in &self.names {
            cases.push(name.to_pattern());
        }
        let cases = cases.iter();
        parse_quote! { #( | #cases )* }
    }

    // the `Expr` to be used if the flag was supplied with an argument
    pub (super) fn get_args_case(&self, var: &Ident, flag_var: &Ident, flag_args: &Ident) -> Result<Expr> {
        let span = self.parser.get().map_or(Span::call_site(), Spanned::span);
        match &self.parser.get() {
            None if self.value.is_set() => Ok(parse_quote_spanned! {span=>
                return Err(TooManyFlagArgs(#flag_var, #flag_args))
            }),
            Some(parser) => {
                let parser_var = new_var("parser");
                Ok(parse_quote_spanned! {span=> {
                        let #parser_var = #parser;
                        #var.set(#parser_var(#flag_args)?)?
                }})
            },
            None => syn_err(var, "Flag is missing any way to be set"),
        }
    }

    // the `Expr` to be used if the flag was supplied with an argument
    pub (super) fn get_noarg_case(&self, var: &Ident, flag_var: &Ident) -> Result<Expr> {
        let span = self.value.get().map_or(Span::call_site(), Spanned::span);
        match &self.value.get() {
            None if self.parser.is_set() => Ok(parse_quote_spanned! {span=>
                return Err(MissingFlagArg(#flag_var))
            }),
            Some(value) => Ok(parse_quote_spanned! {span=>
                #var.set(#value.into())?
            }),
            None => syn_err(var, "Flag is missing any way to be set"),
        }
    }

    pub (super) fn from_attr(attr: &Attribute) -> Result<Self> {
        let mut mods = <Self as Default>::default();
        for arg in parse_flagargs(attr)? {
            mods.apply_modifier(arg)?;
        }
        return Ok(mods);
    }

    fn apply_modifier(&mut self, arg: FlagArg) -> Result<()> {
        use FlagArg::*;
        match arg {
            Long(long) => self.names.push(FlagName::Long(long)),
            Short(short) => self.names.push(FlagName::Short(short)),
            Default(default, span)
                => self.default.syn_set(default, span, "Too many definitions of `default`")?,
            Value(value, span)
                => self.value.syn_set(value, span, "Too many definitions of `value`")?,
            Parser(parser, span)
                => self.parser.syn_set(parser, span, "Too many definitions of `parser`")?,
        }
        return Ok(());
    }
}

impl Spanned for FlagMods {
    fn span(&self) -> Span {
        let mut span = Span::call_site();
        for name in &self.names {
            span = join_spans(span, Some(name.clone()));
        }
        span = join_spans(span, self.default.get());
        span = join_spans(span, self.parser.get());
        span = join_spans(span, self.value.get());
        return span;
    }
}

#[derive(Clone, Debug)]
pub (super) enum FlagName {
    Long(LitStr),
    Short(LitChar),
}

impl FlagName {
    fn to_raw(&self) -> Raw {
        let (prefix, string) = match &self {
            Self::Long(s) => ("--", s.value()),
            Self::Short(c) => ("-", c.value().to_string()),
        };
        return prefix.to_owned() + &string;
    }

    pub fn to_pattern(&self) -> Pat {
        let full = self.to_raw();
        parse_quote_spanned! {self.span()=> #full }
    }
}

impl Spanned for FlagName {
    fn span(&self) -> Span {
        match self {
            Self::Long(lit) => lit.span(),
            Self::Short(lit) => lit.span(),
        }
    }
}

impl PartialEq for FlagName {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw() == other.to_raw()
    }
}

enum FlagArg {
    Long(LitStr),
    Short(LitChar),
    Default(Expr, Span),
    Value(Expr, Span),
    Parser(Expr, Span),
}

type FlagArgs = Punctuated<FlagArg, Token![,]>;

fn parse_flagargs(attr: &Attribute) -> Result<FlagArgs> {
    attr.parse_args_with(FlagArgs::parse_separated_nonempty)
}

fn value_span<T>(input: &ParseStream, span: Span) -> Result<(T, Span)>
    where T: Parse + Spanned
{
    let value: T = input.parse()?;
    let span = span.join(value.span()).unwrap_or(span);
    return Ok((value, span));
}

impl Parse for FlagArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let span = name.span();
        input.parse::<Token![=]>()?;
        match name.to_string().as_str() {
            "long" => {
                let (mut value, span) = value_span::<LitStr>(&input, span)?;
                value.set_span(span);
                Ok(FlagArg::Long(value))
            },
            "short" => {
                let (mut value, span) = value_span::<LitChar>(&input, span)?;
                value.set_span(span);
                Ok(FlagArg::Short(value))
            },
            "default" => {
                let (value, span) = value_span(&input, span)?;
                Ok(FlagArg::Default(value, span))
            },
            "value" => {
                let (value, span) = value_span(&input, span)?;
                Ok(FlagArg::Value(value, span))
            },
            "parser" => {
                let (value, span) = value_span(&input, span)?;
                Ok(FlagArg::Parser(value, span))
            },
            _ => Err(input.error("Unrecognized option modifier")),
        }
    }
}

impl Spanned for FlagArg {
    fn span(&self) -> Span {
        match self {
            FlagArg::Long(val) => val.span(),
            FlagArg::Short(val) => val.span(),
            FlagArg::Default(_, span) => *span,
            FlagArg::Value(_, span) => *span,
            FlagArg::Parser(_, span) => *span,
        }
    }
}

#[cfg(test)]
impl From<char> for FlagName {
    fn from(value: char) -> Self {
        Self::Short(LitChar::new(value, Span::call_site()))
    }
}

#[cfg(test)]
impl From<&str> for FlagName {
    fn from(value: &str) -> Self {
        Self::Long(LitStr::new(value, Span::call_site()))
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! names {
    ($( $val:literal ),*) => {
        vec![ $( $val.into() ),* ]
    };
}

// ########## TESTS ##########

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Pat;

    #[test]
    fn test_get_pattern() {
        let mods = FlagMods {
            names: names!["long1", "long2", 's'],
            ..Default::default()
        };
        let expected_pattern: Pat = parse_quote! {
            | "--long1" | "--long2" | "-s"
        };
        let created_pattern = mods.get_pattern();
        assert_eq!(&expected_pattern, &created_pattern);
    }
}
