use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use rustc_hash::FxHasher;

/// A simple LRU hash.
pub(crate) struct Cache<K: Eq + Debug, V: Debug> {
    items: Vec<(i32, K, V)>,
    max_capacity: usize,
    /// Used to timestamp items in the cache.
    next_stamp: i32,
}

impl<K: Eq + Debug, V: Debug> Cache<K, V> {
    /// Creates a new cache with at most `max_capacity` items.
    pub fn new(max_capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(max_capacity),
            max_capacity,
            next_stamp: 0,
        }
    }

    /// Reads from the cache.
    pub fn read(&mut self, key: &K) -> Option<&V> {
        /*
        #[cfg(debug_assertions)]
        {
            println!("\nCache read:");
            for (hash, key, value) in &self.items {
                println!("- {hash:?} {key:?} {value:?}");
            }
        }
        */

        for i in 0..self.items.len() {
            let item = &self.items[i];
            if &item.1 == key {
                self.next_stamp += 1;
                self.items[i].0 = self.next_stamp;
                return Some(&self.items[i].2);
            }
        }
        None
    }

    /// Writes to the cache.
    pub fn write(&mut self, key: K, value: V) {
        self.next_stamp += 1;
        if self.next_stamp < 0 {
            // Integer overflow. This is a very rare edge case that will appear only in very
            // long-running processes.
            //
            // Since this case is rare, just reset the cache so that `next_stamp` is non-negative
            // again.
            self.items.clear();
            self.next_stamp = 0;
        }

        if self.items.len() >= self.max_capacity {
            // Eviction of oldest member.
            if let Some(i_oldest) = self
                .items
                .iter()
                .enumerate()
                .min_by_key(|(_, (stamp, _, _))| stamp)
                .map(|(i, _)| i)
            {
                self.items.swap_remove(i_oldest);
            }
        }

        // Write without eviction.
        self.items.push((self.next_stamp, key, value));
    }
}

pub(crate) fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = FxHasher::default();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_caching() {
        let mut c: Cache<u64, u64> = Cache::new(3);

        // Empty cache
        assert_eq!(c.read(&1), None);

        // Fill up cache
        c.write(1, 10);
        assert_eq!(c.read(&1), Some(&10));
        c.write(2, 20);
        assert_eq!(c.read(&2), Some(&20));
        c.write(3, 30);
        assert_eq!(c.read(&3), Some(&30));

        // Cache eviction of 1.
        c.write(4, 40);
        assert_eq!(c.read(&1), None);

        // Cache eviction of 3, since 2 is recently read.
        assert_eq!(c.read(&2), Some(&20));
        c.write(5, 50);
        assert_eq!(c.read(&2), Some(&20));

        // Final state.
        assert_eq!(c.read(&1), None);
        assert_eq!(c.read(&2), Some(&20));
        assert_eq!(c.read(&3), None);
        assert_eq!(c.read(&4), Some(&40));
        assert_eq!(c.read(&5), Some(&50));
    }
}
