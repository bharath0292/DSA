use std::collections::HashMap;

fn _max_sum(arr: Vec<i32>, k: usize) -> i32 {
    let n = arr.len();

    if k > n {
        return 0; // not enough elements
    }

    /*
        We sum the first k elements: this is our initial window.
        Example: if arr = [1, 4, 2, 10, 23] and k = 3,
            → windowSum = 1 + 4 + 2 = 7
    */
    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;

    /*
        This is where the sliding window magic happens:
            - arr[i] is the new element entering the window.
            - arr[i-k] is the element exiting the window (from k steps ago).
        By adding one and subtracting one, you avoid recomputing the whole window.

        Example:
            Suppose k = 3, and you already summed arr[0] + arr[1] + arr[2] = 7
            Next window should be arr[1] + arr[2] + arr[3]
            Instead of summing all 3 again,
            Just do: prev_sum - arr[0] + arr[3]
    */
    for i in k..n {
        window_sum += arr[i] - arr[i - k];
        max_sum = max_sum.max(window_sum);
    }

    return max_sum;
}

fn _longest_unique_string(string: &str) -> usize {
    let mut char_index: HashMap<char, usize> = HashMap::new();
    let mut max_length: usize = 0;
    let mut start: usize = 0;

    /*
            If the character has already been seen AND its last seen index is >= current window start,
            then move the start just after the last seen index.
            This effectively skips over the duplicate character.
    */
    for (end, char) in string.chars().enumerate() {
        if let Some(last_index) = char_index.get(&char) {
            if *last_index >= start {
                start += last_index + 1;
            }
        }

        char_index.insert(char, end);

        /*
            Calculate the window length: from `start` to current index `i`
            Update maxLen if this window is longer than any previously found
        */
        max_length = max_length.max(end - start + 1);
    }
    max_length
}
