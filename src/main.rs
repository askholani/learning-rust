// Module declarations
mod problem1;
mod problem2;
mod problem3;
mod problem5;
mod problem6;
mod problem7;

fn main() {
  println!("Hello, world!");
}

#[test]
fn hello_test() {
  println!("Hello, test!");
}
// Tests using the modules
#[test]
fn test_problem5() {
  use problem6::LruCache;

  let mut cache = LruCache::new(2);
  cache.insert("a", 1);
  cache.insert("b", 2);

  assert_eq!(cache.get(&"a"), Some(1));
  assert_eq!(cache.get(&"b"), Some(2));

  cache.insert("c", 3);
  assert_eq!(cache.get(&"a"), None); // "a" should be evicted (LRU)
  assert_eq!(cache.get(&"c"), Some(3));
}
