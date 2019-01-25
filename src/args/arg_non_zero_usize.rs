use std::{
    num::NonZeroUsize,
    str::FromStr
};
use crate::Error;

#[derive(Debug)]
pub struct ArgNonZeroUsize(NonZeroUsize);

impl ArgNonZeroUsize {
    pub fn get(&self) -> NonZeroUsize {
        self.0
    }
}

impl FromStr for ArgNonZeroUsize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ArgNonZeroUsize(NonZeroUsize::new(s.parse()?)
                                        .ok_or_else(|| Error::ArgNotConvertibleToNonZeroUsize(s.to_string()))?))
    }
}
