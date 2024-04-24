use crate::name::Names;

pub struct Help {
    pub (crate) names: Names,
    pub (crate) metavar: Option<String>,
    pub (crate) descr: String,
}

pub fn default_descr() -> String {
    return "Description not set".to_string();
}

pub (crate) fn fmt_names(mut names: Names) -> String {
    let mut s = String::new();
    names.sort();
    for name in names {
        s.push_str(name.to_string().as_str());
    }
    return s;
}

pub fn descr_helper(help: std::option::Option<String>) -> String {
    help.clone().unwrap_or(default_descr())
}

pub (crate) trait HasHelp {
    /// @return `None` if it is to appear hidden
    fn get_help(&self) -> Help;
    fn help(self, help: &str) -> Self;
}
