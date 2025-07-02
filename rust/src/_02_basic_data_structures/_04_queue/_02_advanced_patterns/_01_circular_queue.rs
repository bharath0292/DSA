#[allow(dead_code)]
struct CircularQueue<T> {
    items: Vec<Option<T>>,
    front: usize,
    rear: usize,
    size: usize,
}

#[allow(dead_code)]
impl<T> CircularQueue<T>
where
    T: Clone,
{
    pub fn new(capacity: usize) -> Self {
        CircularQueue {
            items: vec![None; capacity],
            front: 0,
            rear: 0,
            size: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.size == self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Queue is full");
        }

        self.items[self.rear] = Some(item);
        self.rear = self.rear + 1;
        self.size += 1;

        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.items[self.front].take();
        self.front = (self.front + 1) % self.items.len();
        self.size -= 1;

        item
    }
}
