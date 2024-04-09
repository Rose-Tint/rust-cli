/// @type-param T - Represents the result. Typically an enum consisting of all the fields and its value.
///     Might also be `bool` if its just being used as a validator
pub struct Reader<R>(Box<dyn FnOnce(&mut R, Option<String>) -> R>);

impl<R> Reader<R> {
    pub fn optional<F>(f: F) -> Self
        where F: FnOnce(&mut R, Option<String>) -> R + 'static
    {
        Reader(Box::new(f))
    }

    pub fn require<F>(f: F) -> Self
        where F: FnOnce(&mut R, String) -> R + 'static
    {
        Reader(Box::new(move |t, opt| f(t, opt.unwrap_or_default())))
    }

    pub (crate) fn call(self, t: &mut R, opt: Option<String>) -> R {
        return (self.0)(t, opt);
    }
}

impl Reader<bool> {
    pub fn validate<P>(pred: P) -> Self
        where P: FnOnce(String) -> bool + 'static
    {
        return Reader(Box::new(move |_, opt| pred(opt.unwrap_or_default())));
    }
}
