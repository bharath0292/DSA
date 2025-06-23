// Generic Stack implementatio
#[allow(dead_code)]
struct Stack<T> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}
