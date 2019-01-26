use std::{
    ffi::OsString,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    num::ParseIntError,
};

use crate::consts::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    ArgNotConvertibleToUtf8(OsString),
    ArgNotConvertibleToInteger(ParseIntError),
    ArgNotConvertibleToNonZeroUsize(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Error::ArgNotConvertibleToUtf8(os_string) => format!("{}: {:?}",
                                                                 msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8,
                                                                 os_string),
            Error::ArgNotConvertibleToInteger(s) => format!("{}: {:?}",
                                                            msg::ERR_ARG_NOT_CONVERTIBLE_TO_NON_ZERO_USIZE,
                                                            s),
            Error::ArgNotConvertibleToNonZeroUsize(e) => format!("{}: {:?}",
                                                                 msg::ERR_ARG_NOT_CONVERTIBLE_TO_NON_ZERO_USIZE,
                                                                 e),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::ArgNotConvertibleToUtf8(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ArgNotConvertibleToInteger(e)
    }
}
