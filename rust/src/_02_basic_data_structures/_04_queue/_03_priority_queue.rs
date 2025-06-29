use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug)]
struct PriorityItem<T> {
    value: T,
    priority: i32,
}

impl<T> Eq for PriorityItem<T> {}

impl<T> PartialEq for PriorityItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Ord for PriorityItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<T> PartialOrd for PriorityItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct PriorityQueue<T> {
    heap: BinaryHeap<PriorityItem<T>>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn enqueue(&mut self, value: T, priority: i32) {
        self.heap.push(PriorityItem { value, priority });
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.value)
    }
}
