use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn run_program(input: &str) -> String {
    let pkg = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set");
    let exe =
        env::var("CARGO_BIN_EXE_".to_string() + &pkg).expect("CARGO_BIN_EXE_<crate> is not set");

    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run the program");

    child
        .stdin
        .unwrap()
        .write(input.as_bytes())
        .expect("failed to write to stdin");
    child.stdin = None;

    let output = child
        .wait_with_output()
        .expect("failed to read program output");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn example_one_from_spec() {
    assert_eq!(run_program("1\n2\n3\n-1\n"), "6");
}

#[test]
fn example_two_from_spec() {
    assert_eq!(run_program("1\n2\n three\n-1\n"), "NaN");
}

#[test]
fn marker_only() {
    assert_eq!(run_program("-1\n"), "0");
}

#[test]
fn single_number() {
    assert_eq!(run_program("5\n-1\n"), "5");
}

#[test]
fn several_numbers() {
    assert_eq!(run_program("10\n20\n30\n40\n-1\n"), "100");
}

#[test]
fn no_marker_at_all() {
    assert_eq!(run_program("1\n2\n3\n"), "6");
}

#[test]
fn large_positive_numbers() {
    assert_eq!(run_program("123456789\n987654321\n-1\n"), "1111111110");
}

#[test]
fn zero_is_not_positive() {
    assert_eq!(run_program("0\n-1\n"), "NaN");
}

#[test]
fn negative_other_than_marker() {
    assert_eq!(run_program("-5\n-1\n"), "NaN");
}

#[test]
fn non_numeric_token() {
    assert_eq!(run_program("abc\n-1\n"), "NaN");
}

#[test]
fn non_numeric_without_marker() {
    assert_eq!(run_program("three\n"), "NaN");
}

#[test]
fn multiple_numbers_on_one_line() {
    assert_eq!(run_program("1 2\n-1\n"), "NaN");
}

#[test]
fn stops_at_marker_ignores_rest() {
    assert_eq!(run_program("1\n-1\nbad\n"), "1");
}

#[test]
fn leading_spaces() {
    assert_eq!(run_program("  1\n2\n-1\n"), "3");
}

#[test]
fn whitespace_around_non_number() {
    assert_eq!(run_program("1\n  three  \n-1\n"), "NaN");
}

#[test]
fn empty_lines_are_skipped() {
    assert_eq!(run_program("1\n\n2\n-1\n"), "3");
}

#[test]
fn trailing_newline_after_marker() {
    assert_eq!(run_program("1\n2\n-1\n\n"), "3");
}

#[test]
fn exceeds_u64_range() {
    assert_eq!(
        run_program("18446744073709551616\n-1\n"),
        "18446744073709551616"
    );
}

#[test]
fn sum_exceeds_u64_range() {
    assert_eq!(
        run_program("9223372036854775808\n9223372036854775808\n-1\n"),
        "18446744073709551616"
    );
}

#[test]
fn marker_with_leading_whitespace() {
    assert_eq!(run_program("1\n  -1\n"), "1");
}
