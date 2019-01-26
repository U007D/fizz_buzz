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
use std::result::Result as StdResult;

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

pub fn xform(n: usize) -> String {
    // cannot overflow/divide by zero ∵ denominator is constant and non-zero
    #[allow(clippy::integer_arithmetic)]
        match n {
        n if n % 15 == 0 => String::from("fizzbuzz "),
        n if n % 3 == 0 => String::from("fizz "),
        n if n % 5 == 0 => String::from("buzz "),
        n => (n.to_string() + " "),
    }
}

pub fn calc(args: Args, xform: impl Fn(usize) -> String) -> Result<String> {
    Ok(((*args.start).get()..=(*args.end).get()).map(xform)
                                                .collect::<String>()
                                                .trim()
                                                .to_string())
}
