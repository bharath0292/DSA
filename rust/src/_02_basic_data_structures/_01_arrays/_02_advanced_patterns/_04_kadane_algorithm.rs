use std::cmp::max;

pub fn _max_sub_array_sum(arr: Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut current_sum = arr[0];
    let mut max_sum = arr[0];

    for i in 1..arr.len() {
        current_sum = max(arr[i], current_sum + arr[i]);
        max_sum = max(max_sum, current_sum);
    }

    max_sum
}
