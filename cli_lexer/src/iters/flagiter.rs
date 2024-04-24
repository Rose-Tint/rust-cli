use crate::common::*;

pub type FlagArgs = Vec<Raw>;

pub struct FlagIter {
    args: Box<[Raw]>,
    flag_validator: Box<dyn Fn(&Raw) -> bool>,
    index: usize,
    size: usize,
}

impl FlagIter {
    pub fn new(args: Box<[Raw]>, validator: impl Fn(&Raw) -> bool + 'static) -> Self {
        Self {
            flag_validator: Box::new(validator),
            index: 0,
            size: args.len(),
            args,
        }
    }
}

impl Iterator for FlagIter {
    type Item = (Raw, FlagArgs);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size {
            return None;
        }
        let flag = &self.args[self.index];
        self.index += 1;
        let mut args = FlagArgs::new();
        while self.index < self.size {
            let value = &self.args[self.index];
            if (self.flag_validator)(value) {
                break;
            } else {
                args.push((&self.args[self.index]).clone());
                self.index += 1;
            }
        }
        return Some((flag.clone(), args));
    }
}

#[cfg(test)]
mod test {
    use crate::FlagIter;

    macro_rules! string_vec {
        ($( $s:literal ),*) => {
            vec![ $( $s.to_string() ),* ]
        };
    }

    #[test]
    fn test_flag_iter() {
        let args: Box<[String]> = string_vec![
            "flag",
            "flag", "arg1",
            "flag",
            "flag",
            "flag",
            "flag", "arg1", "arg2", "arg3",
            "flag", "arg1", "arg2"
            ].into_boxed_slice();
        let mut iter = FlagIter::new(args, |s| s == "flag");
        let (flag, args) = iter.next()
            .expect("unexpected end of iterator");
        assert_eq!(flag, "flag");
        assert!(args.is_empty());
        let (flag, args) = iter.next()
            .expect("unexpected end of iterator");
        assert_eq!(flag, "flag");
        assert_eq!(args, string_vec!["arg1"]);
        let (flag, args) = iter.next()
            .expect("unexpected end of iterator");
        assert_eq!(flag, "flag");
        assert!(args.is_empty());
        let (flag, args) = iter.next()
            .expect("unexpected end of iterator");
        assert_eq!(flag, "flag");
        assert!(args.is_empty()); 
        let (flag, args) = iter.next()
            .expect("unexpected end of iterator");
        assert_eq!(flag, "flag");
        assert!(args.is_empty());
        let (flag, args) = iter.next()
            .expect("unexpected end of iterator");
        assert_eq!(flag, "flag");
        assert_eq!(args, string_vec!["arg1", "arg2", "arg3"]);
        let (flag, args) = iter.next()
            .expect("unexpected end of iterator");
        assert_eq!(flag, "flag");
        assert_eq!(args, string_vec!["arg1", "arg2"]);
        assert!(iter.next().is_none());
    }
}
