pub(super) use arg_non_zero_usize::ArgNonZeroUsize;
use structopt::StructOpt;

mod arg_non_zero_usize;
#[cfg(test)]
mod unit_tests;

/// Args is a structure representing the user's supplied command-line arguments supplied to the program as an
/// initialized data structure.  Types within the data structure are defined to be as restrictive as possible to
/// ensure only valid inputs are accepted.  Where types accurately reflect the application's constraints, conditional
/// logic, validation code & tests can all be safely elminiated, ∵ the invalid states of concern are not representable.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
    /// Sample argument (can be of any type).  Use `Option<some_type>` for optional arguments
    pub start: ArgNonZeroUsize,
    pub end: ArgNonZeroUsize,
}
