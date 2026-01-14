/**
 * Problem 6 : Generic cache system with interior mutability (Advanced)
 *
 * Description :
 * Implement a generic caching system using interior mutability (RefCall) and smart pointer (Rc). The cache should
 * 1. Work with any type that implements Clone and PartialEq
 * 2. Use LRU (Least Recently Used) eviction policy
 * 3. provide thread-safe interior mutability patterns
 *
 * Requirements:
 * 1. Use Rc<RefCell<T>> for shared mutable access
 * 2. Implement LRU using a combination of HashMap and VecDeque
 * 3. Make it generic over both key and value types
 * 4. Handle cache misses gracefully
 *
 * Function Signature
 * use std::cell::RefCell;
 * use std::collection::{HasMap, VecDeque}
 * use std::hash::Hash;
 * use std::rc::Rc;
 *
 * struct CacheEntry<V> {
 * value: V,
 * access_count : u32
 * }
 *
 * struct LruCache<K,V> where
 * K: Eq + Hash + Clone,
 * V: Clone,
 * {
 * capacity: usize,
 * store : HashMap<K, Rc<RefCell<CacheEntry<V>>>>
 * access_order : VecDeque<K>
 * }
 *
 *  * impl <K, V> LruCache<K,V>
 * where
 * K: Eq + Hash + Clone,
 * V : Clone {
 * fn new (capcity : usize) -> Self {}
 *
 * fn get (&mut self, key &K)-> Option<V> {}
 *
 * fn insert (&mut self, key : K, value : V) {}
 *
 * fn remove (&mut self, key &K)->OPtion<V> {}
 *
 * fn clear (&myt self) {}
 *
 * fn len (&self)-> usize {}
 *
 * fn most_frequent(&self)-> Option<(K,V)> {}
 * }
 *
 * Constraints :
 * 1. Use Rc<RefCall<...>> pattern correctly
 * 2. Update access counts on every read
 * 3. Evict least recently used item when at capacity
 * 4. Handle RefCall borrow erros gracefully (use borrow() and borrow_mut())
 *
 * Example :
 * let mut cache = LruCache::new(2);
 * cache.insert("a",1);
 * cache_insert("b",1);
 *
 * assret_eq!(cache.get(&"a"), Some(1));
 * assert_eq!(cache.get(&"a"),Some(1)); // access_count = 2
 *
 * cache.insert("c",3); // should evict "b" (least recently used)
 * assert_eq!(cache.get(&"b"),None);
 * asset_qa!(cache.get(&"c"), Some(3));
 *
 * Edge cases :
 * 1. Capacity of 0 or 1
 * 2. Inserting same key multiple times
 * 3. Concurrent access patterns (conceptually, though not actually thread-safe)
 * 4. Borrow panics from RefCall
 */
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

pub struct CacheEntry<V> {
  pub value: V,
  pub access_count: u32,
}

pub struct LruCache<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  pub capacity: usize,
  pub store: HashMap<K, Rc<RefCell<CacheEntry<V>>>>,
  pub access_order: VecDeque<K>,
}

impl<K, V> LruCache<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  pub fn new(capacity: usize) -> Self {
    LruCache {
      capacity,
      store: HashMap::new(),
      access_order: VecDeque::new(),
    }
  }

  pub fn get(&mut self, key: &K) -> Option<V> {
    // Step 1: get entry
    let entry = self.store.get(key)?;

    // Step 2: update cache entry
    let mut cache = entry.borrow_mut();
    cache.access_count += 1;
    let value = cache.value.clone();

    // Step 3: update LRU order (AFTER borrow ends)
    drop(cache); // explicit, for clarity

    self.access_order.retain(|k| k != key);
    self.access_order.push_back(key.clone());

    Some(value)
  }

  pub fn insert(&mut self, key: K, value: V) {
    if self.store.contains_key(&key) {
      self.access_order.retain(|k| *k != key);
    } else if self.store.len() == self.capacity {
      if let Some(old_key) = self.access_order.pop_front() {
        self.store.remove(&old_key);
      }
    }

    self.access_order.push_back(key.clone());
    self.store.insert(
      key,
      Rc::new(RefCell::new(CacheEntry {
        value,
        access_count: 0,
      })),
    );
  }

  pub fn remove(&mut self, key: &K) -> Option<V> {
    if let Some(_) = self.access_order.pop_front() {
      self
        .store
        .get(&key)
        .map(|entry| entry.borrow().value.clone())
    } else {
      None
    }
  }

  pub fn clear(&mut self) {
    self.access_order.clear();
    self.store.clear();
  }

  pub fn len(&self) -> usize {
    self.store.len()
  }

  pub fn most_frequent(&self) -> Option<(K, V)> {
    // Get the last key (most recent) from access_order
    if let Some(key) = self.access_order.back() {
      // Look up in store
      if let Some(entry) = self.store.get(key) {
        // Return cloned key and value
        return Some((key.clone(), entry.borrow().value.clone()));
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lru_cache() {
    let mut cache = LruCache::new(2);
    cache.insert("a", 1);
    cache.insert("b", 2);

    assert_eq!(cache.get(&"a"), Some(1));
    assert_eq!(cache.get(&"b"), Some(2));

    cache.insert("c", 3);
    assert_eq!(cache.get(&"a"), None); // "a" should be evicted (LRU)
    assert_eq!(cache.get(&"c"), Some(3));
  }
}
