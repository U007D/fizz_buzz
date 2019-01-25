#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
#![forbid(overflowing_literals)]
//#![deny(warnings)]
//#![deny(missing_docs)]
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
//#![warn(clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented, clippy::use_debug)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
#![allow(clippy::match_bool,)]
use std::{
    num::NonZeroUsize,
    ops::RangeInclusive,
    result::Result as StdResult
};

pub use {
    args::Args,
    consts::*,
    error::Error,
};

mod args;
mod consts;
mod error;
#[cfg(test)]
mod unit_tests;
pub type Result<T> = StdResult<T, Error>;

pub struct FizzBuzz {}

impl FizzBuzz {
    pub fn calc(range: RangeInclusive<NonZeroUsize>) -> String {
        let mut result = String::new();
        let range = range.start().get()..=range.end().get();
        #[allow(clippy::integer_arithmetic)]
        range.for_each(|n| result += &match n {
            n if n % 15 == 0 => String::from("fizzbuzz "),
            n if n % 3 == 0 => String::from("fizz "),
            n if n % 5 == 0 => String::from("buzz "),
            n => (n.to_string() + " "),
        });
        result.trim()
              .to_string()
    }
}

pub fn run(args: Args) -> Result<String> {
    Ok(FizzBuzz::calc(args.start.get()..=args.end.get()))
}
