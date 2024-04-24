use std::fmt::Debug;
use quote::ToTokens;

use crate::utils::Spanned;


#[derive(Clone, Copy, Default, Debug, PartialEq)]
enum SetOnceInner<T> {
    #[default]
    NotSet,
    Set(T),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetOnce<T>(SetOnceInner<T>);

use SetOnceInner::*;

impl<T> SetOnce<T> {
    pub fn new() -> Self {
        Self(NotSet)
    }

    pub fn set<E>(&mut self, value: T, err: E) -> std::result::Result<(), E> {
        match self.0 {
            NotSet => self.0 = Set(value),
            Set(_) => return Err(err),
        }
        return Ok(());
    }

    pub fn syn_set(&mut self, value: T, span: impl Spanned, msg: &str) -> syn::Result<()> {
        self.set(value, syn::Error::new(span.span(), msg))
    }

    pub fn get(&self) -> Option<&T> {
        match &self.0 {
            NotSet => None,
            Set(value) => Some(value),
        }
    }

    pub fn map<F, U>(&self, f: F) -> SetOnce<U>
        where F: FnOnce(&T) -> U
    {
        match &self.0 {
            NotSet => SetOnce(NotSet),
            Set(val) => SetOnce(Set(f(val))),
        }
    }

    pub fn iter(&self) -> Iter<T>
        where T: Clone
    {
        match &self.0 {
            NotSet => Iter(None),
            Set(value) => Iter(Some(value.clone())),
        }
    }

    pub fn is_set(&self) -> bool {
        match &self.0 {
            NotSet => false,
            Set(_) => true,
        }
    }
}

impl<T> Default for SetOnce<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Into<Option<T>> for SetOnce<T> {
    fn into(self) -> Option<T> {
        match self.0 {
            NotSet => None,
            Set(value) => Some(value),
        }
    }
}

impl<T: ToTokens> ToTokens for SetOnce<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.get().to_tokens(tokens)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Iter<T>(Option<T>);

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
    }
}
