use std::collections::VecDeque;

use key::Key;

/// Stores a part of values, with keys at distance between `2^k` and
/// `2^(k+1)` of the table's main key.
pub struct Bucket<TKey: Key, TValue> {
    data: VecDeque<(TKey, TValue)>,
    max_size: usize,
}

impl<TKey: Key, TValue> Bucket<TKey, TValue> {
    /// Creates a new bucket.
    pub fn new(max_size: usize) -> Bucket<TKey, TValue> {
        Bucket {
            data: VecDeque::with_capacity(max_size),
            max_size: max_size,
        }
    }

    /// Adds a new item to the bucket.
    /// If the bucket is already full, pops the oldest element before
    /// insertion, and returns it.
    /// If there is already an element with that key, pops it before
    /// insertion, and returns it.
    pub fn update(&mut self, key: TKey, value: TValue) -> Option<(TKey, TValue)> {
        // Search if there is already an element with that key.
        let mut index_to_remove = None;
        for (i, k) in self.data.iter().map(|&(ref k,_)| k.clone()).enumerate() {
            if k == key {
                index_to_remove = Some(i);
            }
        }

        // If there is already an element with that key, pop it and
        // get its value.
        let res = {
            if let Some(i) = index_to_remove {
                self.data.remove(i)
            }
            else if self.data.len() == self.max_size {
                self.data.pop_front()
            }
            else {
                None
            }
        };

        // Insert and return
        self.data.push_back((key, value));
        res
    }

    /// Returns the content of this bucket.
    pub fn data(&self) -> &VecDeque<(TKey, TValue)> {
        &self.data
    }
}
