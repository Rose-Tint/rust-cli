pub mod result;
pub use result::*;

/// @type-param T - Represents the result. Typically an enum consisting of all
///     the fields and its value. Might also be `bool` if its just being used
///     as a validator
pub struct Reader<T>(Box<dyn FnOnce(Option<String>) -> Result<T>>);

impl<T> Reader<T> {
    pub fn new(f: impl FnOnce(Option<String>) -> Result<T> + 'static) -> Self {
        Reader(Box::new(f))
    }

    pub fn no_arg(value: T) -> Self
        where T: 'static
    {
        Reader(Box::new(|opt| match opt {
            None => Ok(value),
            _ => Err(ArgsNotAllowed),
        }))
    }

    pub (crate) fn read(self, opt: Option<String>) -> Result<T> {
        (self.0)(opt)
    }

    pub fn optional(default: T, f: impl FnOnce(String) -> Result<T> + 'static) -> Self
        where T: 'static
    {
        Reader::new(|opt| match opt {
            None => Ok(default),
            Some(opt) => f(opt),
        })
    }
}

impl<T> Default for Reader<T>
    where T: Default + From<String> + 'static {
    fn default() -> Self {
        Reader::new(|opt| match opt {
            None => Ok(T::default()),
            Some(opt) => Ok(opt.into()),
        })
    }
}

impl<T, F> From<F> for Reader<T>
    where F: FnOnce(Option<String>) -> Result<T> + 'static
{
    fn from(f: F) -> Self {
        Reader(Box::new(f))
    }
}
