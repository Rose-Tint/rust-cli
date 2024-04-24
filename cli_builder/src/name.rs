#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FlagName {
    /// Only a long-form name. `Long("example")` is activated by `--example`
    Long(String),
    /// Only a one-letter name. `Short('e')` is activated by `-e`
    Short(char),
}

impl From<char> for FlagName {
    fn from(value: char) -> Self {
        FlagName::Short(value)
    }
}

impl From<&str> for FlagName {
    /// Converts to `Long` for internal reasons (this should only be used for
    /// `Option`s).
    fn from(value: &str) -> Self {
        FlagName::Long(value.to_string())
    }
}

impl ToString for FlagName {
    fn to_string(&self) -> String {
        match self {
            Self::Long(name) => format!("--{name}"),
            Self::Short(name) => format!("-{name}"),
        }
    }
}

pub type Names = Vec<FlagName>;
