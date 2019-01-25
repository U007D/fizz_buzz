#![allow(clippy::option_unwrap_used)]
use super::*;

#[test]
fn calc_from_1_to_1_yields_1() {
    // setup
    let start = NonZeroUsize::new(1).unwrap();
    let end = NonZeroUsize::new(1).unwrap();
    let range = RangeInclusive::new(start, end);
    let expected_result = "1";

    // given a `FizzBuzz::calc` function
    let sut = FizzBuzz::calc;

    // when it is invoked
    let result = sut(range);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_2_yields_1_2() {
    // setup
    let start = NonZeroUsize::new(1).unwrap();
    let end = NonZeroUsize::new(2).unwrap();
    let range = RangeInclusive::new(start, end);
    let expected_result = "1 2";

    // given a `FizzBuzz::calc` function
    let sut = FizzBuzz::calc;

    // when it is invoked
    let result = sut(range);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_3_yields_1_2_fizz() {
    // setup
    let start = NonZeroUsize::new(1).unwrap();
    let end = NonZeroUsize::new(3).unwrap();
    let range = RangeInclusive::new(start, end);
    let expected_result = "1 2 fizz";

    // given a `FizzBuzz::calc` function
    let sut = FizzBuzz::calc;

    // when it is invoked
    let result = sut(range);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_5_yields_1_2_fizz_4_buzz() {
    // setup
    let start = NonZeroUsize::new(1).unwrap();
    let end = NonZeroUsize::new(5).unwrap();
    let range = RangeInclusive::new(start, end);
    let expected_result = "1 2 fizz 4 buzz";

    // given a `FizzBuzz::calc` function
    let sut = FizzBuzz::calc;

    // when it is invoked
    let result = sut(range);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}

#[test]
fn calc_from_1_to_16_yields_1_2_etc_14_fizzbuzz_16() {
    // setup
    let start = NonZeroUsize::new(1).unwrap();
    let end = NonZeroUsize::new(16).unwrap();
    let range = RangeInclusive::new(start, end);
    let expected_result = "1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz 16";

    // given a `FizzBuzz::calc` function
    let sut = FizzBuzz::calc;

    // when it is invoked
    let result = sut(range);

    // then it should yield the expected result
    assert_eq!(result, expected_result);
}
