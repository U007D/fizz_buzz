use super::*;

#[test]
fn args_cannot_be_constructed_with_a_0_start_value() {
    // setup
    let app = "args::unit_test";
    let start = "0";
    let end = "1";
    let args = vec![app, start, end];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert_eq!(result.is_err(), true);
}

#[test]
fn args_cannot_be_constructed_with_a_0_end_value() {
    // setup
    let app = "args::unit_test";
    let start = "2";
    let end = "0";
    let args = vec![app, start, end];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert_eq!(result.is_err(), true);
}
