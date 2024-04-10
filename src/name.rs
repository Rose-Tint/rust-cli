#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub (crate) enum Name {
    /// Just a regular name, typically only used for commands
    Unprefixed(String),
    /// Only a long-form name. `Long("example")` is activated by `--example`
    Long(String),
    /// Only a one-letter name. `Short('e')` is activated by `-e`
    Short(char),
}

impl From<char> for Name {
    fn from(value: char) -> Self {
        Name::Short(value)
    }
}

impl From<&str> for Name {
    /// Converts to `Long` for internal reasons (this should only be used for
    /// `Option`s).
    fn from(value: &str) -> Self {
        Name::Long(value.to_string())
    }
}

impl ToString for Name {
    fn to_string(&self) -> String {
        match self {
            Self::Unprefixed(name) => name.clone(),
            Self::Long(name) => format!("--{name}"),
            Self::Short(name) => format!("-{name}"),
        }
    }
}

pub (crate) type Names = Vec<Name>;
