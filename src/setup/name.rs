#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Name {
    /// Only a long-form name. `Long("example")` is activated by `--example`
    Long(String),
    /// Only a one-letter name. `Short('e')` is activated by `-e`
    Short(char),
}

pub type Names = Vec<Name>;
