fn _calculate_prefix_sum(arr: Vec<i32>) -> Vec<i32> {
    let mut prefix_sum = vec![0; arr.len()];
    prefix_sum[0] = arr[0];

    for i in 1..arr.len() {
        prefix_sum[i] = prefix_sum[i - 1] + arr[i]
    }
    prefix_sum
}

fn _range_sum(prefix_sum: &[i32], left: usize, right: usize) -> i32 {
    if left == 0 {
        return 0;
    }

    prefix_sum[right] - prefix_sum[left - 1]
}
