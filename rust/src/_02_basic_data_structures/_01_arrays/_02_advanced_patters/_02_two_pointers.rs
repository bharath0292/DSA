fn _find_pair_sum(arr: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let (mut left, mut right) = (0, arr.len() - 1);

    let mut sum;
    while left < right {
        sum = arr[left] + arr[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    None
}

fn _remove_duplicates(arr: &mut Vec<i32>) -> usize {
    // If the input array is empty, there's nothing to process, so return 0.
    if arr.is_empty() {
        return 0;
    }

    // writePtr is used to keep track of where the next unique value should be placed.
    // Index 0 is always considered unique, so we start at index 1.
    let mut write_ptr = 1;

    /*
        - readPtr scans through the array starting from index 1.
        - At each step:
            * Compare current value arr[readPtr] with previous value arr[readPtr - 1].
            * If they are different, that means a new unique element is found:
                ~ Copy arr[readPtr] to arr[writePtr].
                ~ Move writePtr forward.
        - If values are the same, it skips them (thus removing duplicates logically).
    */
    for read_ptr in 1..arr.len() {
        if arr[read_ptr] != arr[read_ptr - 1] {
            arr[write_ptr] = arr[read_ptr];
            write_ptr += 1;
        }
    }

    write_ptr
}
