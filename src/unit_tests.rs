#![allow(clippy::option_unwrap_used)]
use super::*;
use std::num::NonZeroUsize;
use args::ArgNonZeroUsize;

#[test]
fn calc_from_1_to_1_yields_1() {
    // setup
    let start = ArgNonZeroUsize(NonZeroUsize::new(1).unwrap());
    let end = ArgNonZeroUsize(NonZeroUsize::new(1).unwrap());
    let args = Args { start, end };
    let xform = xform;
    let expected_result = Ok(String::from("1"));

    // given a `calc` function
    let sut = calc;

    // when it is invoked
    let result = sut(args, xform);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_2_yields_1_2() {
    // setup
    let start = ArgNonZeroUsize(NonZeroUsize::new(1).unwrap());
    let end = ArgNonZeroUsize(NonZeroUsize::new(2).unwrap());
    let args = Args { start, end };
    let expected_result = Ok(String::from("1 2"));

    // given a `calc` function
    let sut = calc;

    // when it is invoked
    let result = sut(args, xform);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_3_yields_1_2_fizz() {
    // setup
    let start = ArgNonZeroUsize(NonZeroUsize::new(1).unwrap());
    let end = ArgNonZeroUsize(NonZeroUsize::new(3).unwrap());
    let args = Args { start, end };
    let expected_result = Ok(String::from("1 2 fizz"));

    // given a `calc` function
    let sut = calc;

    // when it is invoked
    let result = sut(args, xform);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_5_yields_1_2_fizz_4_buzz() {
    // setup
    let start = ArgNonZeroUsize(NonZeroUsize::new(1).unwrap());
    let end = ArgNonZeroUsize(NonZeroUsize::new(5).unwrap());
    let args = Args { start, end };
    let expected_result = Ok(String::from("1 2 fizz 4 buzz"));

    // given a `calc` function
    let sut = calc;

    // when it is invoked
    let result = sut(args, xform);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_16_yields_1_2_etc_14_fizzbuzz_16() {
    // setup
    let start = ArgNonZeroUsize(NonZeroUsize::new(1).unwrap());
    let end = ArgNonZeroUsize(NonZeroUsize::new(16).unwrap());
    let args = Args { start, end };
    let expected_result = Ok(String::from("1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz 16"));

    // given a `calc` function
    let sut = calc;

    // when it is invoked
    let result = sut(args, xform);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}
