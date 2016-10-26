#![cfg(test)]
use problem1::{sum, dedup, filter};

#[test]
fn test_sum_small() {
    let array = [1, 2, 3, 4, 5];
    assert_eq!(sum(&array), 15);
}


#[test]
fn test_sum_with_negative() {
    let array = [1, 2, 3, 4, -5];
    assert_eq!(sum(&array), 5);
}


#[test]
fn test_dedup_small() {
    let vs = vec![1, 2, 2, 3, 4, 1];
    assert_eq!(dedup(&vs),  vec![1, 2, 3, 4]);
}

#[test]
fn test_dedup_medium() {
    let vs = vec![1,1,1, 2, 2, 3,1, 4,5, 1];
    assert_eq!(dedup(&vs),  vec![1, 2, 3, 4,5]);
}


fn even_predicate(x: i32) -> bool {
    (x % 2) == 0
}

#[test]
fn test_filter_small() {
    let vs = vec![1, 2, 3, 4, 5];
    assert_eq!(filter(&vs, &even_predicate), vec![2, 4]);
}