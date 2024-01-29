use lesson12::bsearch;

mod common;

const ARR: [i32; 10] = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];

#[test]
fn test_bsearch() {
    common::setup();

    let desired_value = 10;
    let result = bsearch(&ARR, &desired_value).unwrap();

    assert_eq!(result, (desired_value, 5));
}

#[test]
fn test_bsearch_smallest_found() {
    let desired_value = ARR.first().unwrap();
    let result = bsearch(&ARR, desired_value).unwrap();

    assert_eq!(result, (desired_value.to_owned(), 0));
}

#[test]
fn test_bsearch_largest_found() {
    let desired_value = ARR.last().unwrap();
    let result = bsearch(&ARR, desired_value).unwrap();

    assert_eq!(result, (desired_value.to_owned(), ARR.len() - 1));
}

#[test]
fn test_bsearch_smallest_not_found() {
    let desired_value = -2;
    let result = bsearch(&ARR, &desired_value);

    assert!(result.is_none());
}

#[test]
fn test_bsearch_largest_not_found() {
    let desired_value = 200;
    let result = bsearch(&ARR, &desired_value);

    assert!(result.is_none());
}
