pub fn _binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = arr.len();

    while left <= right {
        let mid = (left + right) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => {
                return Some(mid);
            }
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }

    None
}
