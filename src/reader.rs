pub type RdrResult<R> = Result<R, String>;

/// @type-param T - Represents the result. Typically an enum consisting of all the fields and its value.
///     Might also be `bool` if its just being used as a validator
pub struct Reader<O, T>(Box<dyn FnOnce(O, T) -> RdrResult<O>>);

impl<O, T> Reader<O, T> {
    pub fn new<F>(f: F) -> Self
        where F: FnOnce(O, T) -> RdrResult<O> + 'static
    {
        Reader(Box::new(f))
    }

    pub (crate) fn call(self, t: O, opt: T) -> RdrResult<O> {
        return (self.0)(t, opt);
    }
}

impl<O, T> Reader<O, Option<T>> {
    pub fn require<F>(f: F) -> Self
        where F: FnOnce(O, T) -> RdrResult<O> + 'static
    {
        return Reader(Box::new(|opts, opt|
            match opt {
                None => Err("Missing required argument $m".to_string()),
                Some(x) => f(opts, x),
            }
        ));
    }
}

impl<O> Reader<O, Option<String>> {
    pub fn bool_reader<F>(f: F) -> Self
        where F: FnOnce(O, bool) -> RdrResult<O> + 'static
    {
        return Reader(Box::new(|opts, val|
            if let Some(mut s) = val {
                s = s.trim().to_lowercase();
                if s == "true" {
                    f(opts, true)
                } else if s == "false" {
                    f(opts, false)
                } else {
                    Err("Invalid Argument".to_string())
                }
            } else {
                f(opts, true)
            }));
    }
}
