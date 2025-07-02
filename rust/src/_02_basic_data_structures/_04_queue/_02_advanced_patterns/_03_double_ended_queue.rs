use std::collections::VecDeque;

#[allow(dead_code)]
struct Dequeu<T> {
    items: VecDeque<T>,
}

#[allow(dead_code)]
impl<T> Dequeu<T> {
    fn new() -> Self {
        Dequeu {
            items: VecDeque::new(),
        }
    }

    fn add_front(&mut self, item: T) {
        self.items.push_front(item);
    }

    fn add_rear(&mut self, item: T) {
        self.items.push_back(item);
    }

    fn remove_front(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    fn remove_rear(&mut self) -> Option<T> {
        self.items.pop_back()
    }
}
