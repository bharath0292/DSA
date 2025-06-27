// Generic Stack implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
