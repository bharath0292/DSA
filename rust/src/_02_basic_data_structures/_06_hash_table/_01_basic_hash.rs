use std::hash::{DefaultHasher, Hash, Hasher};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct HashNode<K, V> {
    key: K,
    value: V,
    next: Option<Box<HashNode<K, V>>>,
}

#[allow(dead_code)]
struct HashTable<K, V> {
    buckets: Vec<Option<Box<HashNode<K, V>>>>,
    size: usize,
    capacity: usize,
}

#[allow(dead_code)]
impl<K: Hash + Eq + Clone, V: Clone> HashTable<K, V> {
    fn new(capacity: usize) -> Self {
        HashTable {
            buckets: vec![None; capacity],
            size: 0,
            capacity,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    fn resize(&mut self) {
        let old_buckets = std::mem::replace(&mut self.buckets, vec![None; self.capacity * 2]);
        self.capacity *= 2;
        self.size = 0;

        for bucket in old_buckets.into_iter().flatten() {
            let mut node = bucket;
            loop {
                self.insert(node.key, node.value);
                if let Some(next) = node.next {
                    node = next;
                } else {
                    break;
                }
            }
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);

        let mut current = &mut self.buckets[index];
        while let Some(node) = current {
            if node.key == key {
                node.value = value;
                return;
            }
            current = &mut node.next;
        }

        let new_node = Box::new(HashNode {
            key,
            value,
            next: self.buckets[index].take(),
        });
        self.buckets[index] = Some(new_node);
        self.size += 1;

        if (self.size / self.capacity) as f64 > 0.75 {
            self.resize()
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        let index = self.hash(&key);
        let mut current = &self.buckets[index];

        while let Some(node) = current {
            if node.key == key {
                return Some(&node.value);
            }
            current = &node.next
        }

        None
    }
}
