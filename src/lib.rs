pub use cli_builder as builder;
pub use cli_builder::derives;
pub use cli_derive_macros::*;
pub use cli_lexer::err;

pub mod derive {
    pub use cli_derive::*;
}

/// export is required for derivation.
/// contains the items used by the derive-macros.
pub mod internal {
    pub use cli_builder::derives::*;
    pub use cli_lexer::internal::*;
    pub use cli_lexer::err::CmdLineErr::{self, *};
    pub use cli_lexer::*;
}
