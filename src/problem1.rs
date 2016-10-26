/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for i in slice {
        sum += *i;
    }
    sum
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut return_arr : Vec<i32> = Vec::new();

    for i in vs {
        if !vec_contains(*i, return_arr.clone()) {
            return_arr.push(*i);
        }
    }
    return return_arr;
}

fn vec_contains(i: i32, vs: Vec<i32>) -> bool {
    println!("i:{}", i);
    for v in vs {
        if v == i {
            return true;
        }
    }
    return false;
}


/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    // TODO
    let mut return_arr: Vec<i32> = Vec::new();

    for v in vs {
        if pred(*v) {
            return_arr.push(*v);
        }
    } 
    return return_arr;
}