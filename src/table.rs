use std::cmp;

use key::Key;
use bucket::Bucket;

/// Key-Value store.
pub struct Table<TKey: Key, TValue> {
    buckets: Vec<Bucket<TKey, TValue>>,
    my_key: TKey,
}

impl<TKey: Key, TValue> Table<TKey, TValue> {
    /// Creates a new Table instance.
    pub fn new(my_key: TKey, max_bucket_size: usize, max_bits: usize) -> Table<TKey, TValue> {
        let buckets = (0..max_bits+1).map(|_| Bucket::new(max_bucket_size)).collect();
        Table {
            buckets: buckets,
            my_key: my_key,
        }
    }

    /// Returns the index of the buck`t where the given `key` should be placed.
    fn bucket_index(&self, key: &TKey) -> usize {
        key.bitxor(&self.my_key).bits()
    }

    /// Inserts a new item in the table.
    /// See [`Bucket::update()`] for details about the return value.
    ///
    /// [`Bucket::update()`]: struct.Bucket.html#method.update
    pub fn update(&mut self, key: TKey, value: TValue) -> Option<(TKey, TValue)> {
        // Compute the number of bits of self.my_key ^ key
        let bucket_index = self.bucket_index(&key);

        // Get the bucket the element should be put in.
        let bucket = self.buckets.get_mut(bucket_index).expect(
                &format!("Distance between {:?} and {:?} is greater than 2^max_bits.",
                self.my_key, key));

        // Delegate the work of insertion to the bucket.
        bucket.update(key, value)
    }

    /// Returns `count` nodes closest to the `target`, sorted in
    /// increasing distance to the target.
    pub fn find(&self, target: &TKey, count: usize) -> Vec<(TKey, &TValue)> {
        let mut data_copy: Vec<(TKey, &TValue)> = self.buckets.iter().flat_map(|b| {
            b.data().iter().map(|&(ref k, ref v)| (k.clone(), v))
        }).collect();
        data_copy.sort_by_key(|&(ref k, _)| target.bitxor(k));
        data_copy[0..cmp::min(count, data_copy.len())].to_vec()
    }
}
