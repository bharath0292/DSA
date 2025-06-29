#[allow(dead_code)]
pub struct Queue<T> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            Some(self.items.remove(0))
        } else {
            None
        }
    }

    fn front(&self) -> Option<&T> {
        self.items.first()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}
