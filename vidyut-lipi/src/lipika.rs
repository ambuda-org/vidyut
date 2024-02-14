//! Provides a convenient transliteration API for end users.

use crate::mapping::Mapping;
use crate::scheme::Scheme;
use crate::transliterate::transliterate;

// Size of the internal `Vec` cache. We search this cache with a linear scan, so keep this small.
const CACHE_CAPACITY: usize = 10;

/// A `Mapping` as stored in `Lipika`'s internal cache.
///
/// While creating a `Mapping` is cheap, doing so repeatedly within an inner loop will add some
/// unnecessary overhead. So, cache common mappings so that callers can reuse them. Essentially, we
/// are memoizing the `Mapping` constructor.
#[derive(Clone, Eq, PartialEq)]
struct CachedMapping {
    /// A "timestamp" that represents when the mapping was last used.
    stamp: i32,
    /// The source scheme.
    from: Scheme,
    /// The destination scheme.
    to: Scheme,
    /// The mapping between `from` and `to`.
    mapping: Mapping,
}

/// A convenient and high-performance transliterator.
///
/// `Lipika` is a thin wrapper over the low-level `transliterate_with_mapping` function, which
/// performs transliteration with the help of a `Mapping` struct.
///
/// Each `Lipika` instance has its own internal LRU cache, and this cache stores roughly 10
/// `Mapping`s at most.
///
/// If you need more precise control, please use `transliterate_with_mapping` directly.
///
/// ### Usage
///
/// ```
/// use vidyut_lipi::{Lipika, Scheme, detect};
///
/// // `Lipika` must be `mut` since its method calls mutate its internal cache.
/// let mut lipika = Lipika::new();
///
/// let deva = lipika.transliterate("saMskRtam", Scheme::HarvardKyoto, Scheme::Devanagari);
/// assert_eq!(deva, "संस्कृतम्");
///
/// let iast = lipika.transliterate("saMskRtam", Scheme::HarvardKyoto, Scheme::Iast);
/// assert_eq!(iast, "saṃskṛtam");
///
/// let detected = detect(&deva).unwrap_or(Scheme::HarvardKyoto);
/// let original = lipika.transliterate(deva, detected, Scheme::HarvardKyoto);
/// assert_eq!(original, "saMskRtam");
/// ```
#[derive(Clone, Default, Eq, PartialEq)]
pub struct Lipika {
    cache: Vec<CachedMapping>,
    // Indicates when a mapping was last used.
    //
    // Use an i32 so that we can safely overflow (if in a long-running process)
    next_stamp: i32,
}

impl Lipika {
    /// Creates a new `Lipika` instance with an empty cache.
    pub fn new() -> Self {
        Self {
            cache: Vec::new(),
            next_stamp: 0,
        }
    }

    /// Transliterates the given input text.
    ///
    /// `transliterate` first checks if the mapping between `from` and `to` is available in the
    /// internal cache. If the mapping exists, `transliterate` will reuse it. Otherwise,
    /// `transliterate` will create a new mapping and store it for future use.
    ///
    /// For details on the underlying algorithm, see the comments on the `transliterate` function.
    pub fn transliterate(&mut self, input: impl AsRef<str>, from: Scheme, to: Scheme) -> String {
        let mapping = self.find_or_create_mapping(from, to);
        transliterate(input.as_ref(), mapping)
    }

    /// Finds an existing mapping to reuse, or creates one if absent.
    ///
    /// This code assumes that a `Mapping` is a pure function of `from` and `to`.
    fn find_or_create_mapping(&mut self, from: Scheme, to: Scheme) -> &Mapping {
        self.next_stamp += 1;
        if self.next_stamp < 0 {
            // Integer overflow. This is a very rare edge case that will appear only in very
            // long-running processes.
            //
            // Since this case is rare, just reset the cache so that `next_stamp` is non-negative
            // again.
            self.cache.clear();
            self.next_stamp = 0;
        }

        // Check the cache. For now, assume that a `Mapping` is a pure function of `from` and `to`.
        if let Some(i) = self.cache.iter().position(|x| x.from == from && x.to == to) {
            // Cache hit.
            self.cache[i].stamp += self.next_stamp;
            &self.cache[i].mapping
        } else {
            // Cache miss.
            if self.cache.len() >= CACHE_CAPACITY {
                // At capacity --> evict.
                if let Some(i_oldest) = self
                    .cache
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, x)| x.stamp)
                    .map(|(i, _)| i)
                {
                    self.cache.swap_remove(i_oldest);
                }
            }

            let entry = CachedMapping {
                stamp: self.next_stamp,
                from,
                to,
                mapping: Mapping::new(from, to),
            };
            self.cache.push(entry);
            &self.cache.last().expect("just pushed").mapping
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut lipika = Lipika::new();
        assert!(lipika.cache.is_empty());

        let mut num_calls = 0;
        for scheme in Scheme::iter() {
            let result = lipika.transliterate("saMskRtam", Scheme::HarvardKyoto, *scheme);
            assert!(!result.is_empty());
            num_calls += 1;
        }

        // Cache should now be full.
        assert_eq!(lipika.cache.len(), CACHE_CAPACITY);
        assert_eq!(lipika.next_stamp, num_calls);
    }
}
