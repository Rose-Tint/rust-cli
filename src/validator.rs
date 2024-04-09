extern crate regex;
use regex::RegexBuilder;


pub struct Validator(Box<dyn FnOnce(Option<String>) -> bool>);

impl Validator {
    pub fn new<F>(f: F) -> Self
        where F: FnOnce(Option<String>) -> bool + 'static
    {
        return Validator(Box::new(f));
    }

    pub fn regex(pattern: &str) -> Self {
        let re = RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build().unwrap();
        return Self::new(
            move |opt| re.is_match(opt.unwrap_or_default().as_str())
        );
    }

    pub fn require<F>(f: F) -> Self
        where F: FnOnce(String) -> bool + 'static
    {
        return Self::new(|opt| opt.map_or(false, f));
    }

    pub fn optional<F>(f: F) -> Self
        where F: FnOnce(String) -> bool + 'static
    {
        return Self::new(|opt| opt.map_or(true, f));
    }

    #[allow(dead_code)]
    pub (crate) fn validate(self, s: Option<String>) -> bool {
        return (self.0)(s);
    }
}
