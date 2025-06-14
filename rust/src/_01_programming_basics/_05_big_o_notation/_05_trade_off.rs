use std::collections::HashMap;

fn _fibonacci_memoized(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&value) = memo.get(&n) {
        return value;
    }

    if n <= 1 {
        return n;
    }

    let value = _fibonacci_memoized(n - 1, memo) + _fibonacci_memoized(n - 2, memo);
    memo.insert(n, value);

    value
}
