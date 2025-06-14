// O(n²) time complexity
pub fn _bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut swapped = true;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true
            }
        }

        if !swapped {
            break;
        }
    }
}

// O(n²) space complexity
fn _create_matrix(n: usize) -> Vec<Vec<i32>> {
    vec![vec![0; n]; n]
}
