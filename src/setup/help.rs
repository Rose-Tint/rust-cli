use super::Names;

pub struct Help {
    pub (crate) names: Names,
    pub (crate) metavar: Option<String>,
    pub (crate) descr: String,
}

pub fn default_help() -> String {
    return "Description not set".to_string();
}

pub (crate) trait HasHelp {
    /// @return `None` if it is to appear hidden
    fn get_help(&self) -> Help;
    fn help(self, help: &str) -> Self;
}

