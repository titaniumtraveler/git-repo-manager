use bstr::BString;

pub use self::tokenize::*;

mod tokenize;

pub struct Template(BString);

impl Template {
    pub fn new(str: impl Into<BString>) -> Self {
        Self(str.into())
    }

    pub fn tokenize(&self) -> Tokenize {
        Tokenize(self.0.as_ref())
    }
}
