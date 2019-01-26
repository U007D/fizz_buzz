use std::{
    num::NonZeroUsize,
    ops::Deref,
    str::FromStr
};
use crate::Error;

#[derive(Debug)]
pub struct ArgNonZeroUsize(pub(crate) NonZeroUsize);

impl Deref for ArgNonZeroUsize {
    type Target = NonZeroUsize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for ArgNonZeroUsize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ArgNonZeroUsize(NonZeroUsize::new(s.parse()?)
                                        .ok_or_else(|| Error::ArgNotConvertibleToNonZeroUsize(s.to_string()))?))
    }
}
