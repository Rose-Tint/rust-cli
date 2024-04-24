use crate::err::{ParseResult, *};
use crate::common::*;


/// a helper for inilializing flags to allow for required flags
pub enum FlagInit<T> {
    Required(Option<T>, Raw /* flag name */),
    Default(T),
}

impl<T> FlagInit<T> {
    pub fn required(flag_name: &'static str) -> Self {
        Self::Required(None, flag_name.to_string())
    }

    pub fn default(value: T) -> Self {
        Self::Default(value)
    }

    #[must_use]
    pub fn unwrap(self) -> ParseResult<T> {
        match self {
            Self::Required(None, flag_name) => Err(MissingFlag(flag_name)),
            Self::Required(Some(value), _) => Ok(value),
            Self::Default(value) => Ok(value),
        }
    }

    #[must_use]
    pub fn set(&mut self, value: T) -> ParseResult<()> {
        match self {
            Self::Required(Some(_), flag_name)
                => return Err(FlagSetTwice(flag_name.to_string())),
            Self::Required(ref mut val, _) => *val = Some(value),
            Self::Default(ref mut val) => *val = value,
        };
        return Ok(());
    }
}
