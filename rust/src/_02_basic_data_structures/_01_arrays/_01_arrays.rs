fn _array_examples() {
    // Fixed size array
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Vector (dynamic array)
    let mut vec = Vec::new();
    vec.push(1);

    // Vector with capacity
    let mut _vec: Vec<i32> = Vec::with_capacity(5);

    // Initialize vector with values
    let _vec = vec![1, 2, 3, 4, 5];

    // 2D vector
    let _matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
}

// Common vector operations
fn _vector_operations() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Add elements
    numbers.push(6);

    // Remove and return last element
    let _last = numbers.pop();

    // Insert at index
    numbers.insert(1, 10);

    // Remove at index
    numbers.remove(2);

    // Slice
    let _slice = &numbers[1..4];

    // Extend from slice
    numbers.extend_from_slice(&[6, 7, 8]);
}

// Vector patterns
fn _vector_patterns() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Filter
    let _even_nums: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();

    // Map
    let _doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();

    // Reduce
    let _sum: i32 = numbers.iter().sum();

    // Chaining operation
    let _result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
}

// Custom collection implementation
#[allow(dead_code)]
struct CustomVec<T> {
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> CustomVec<T> {
    pub fn new(&self) -> Self {
        CustomVec { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}
