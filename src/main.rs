fn main() {
  println!("Hello, world!");
}

#[test]
fn hello_test() {
  println!("Hello, test!");
}

// learning rust based on hackrrank's styles
// Problem 1: Array Statistics (Easy)
/**
 * Description:
Given a fixed-size array of integers, calculate:

1. The sum of all elements

2. The maximum value

3. The minimum value

4. The average (as floating-point)

fn array_stats(arr: [i32; 5]) -> (i32, i32, i32, f64)

 */

fn array_stats(arr: [i32; 5]) -> (i32, i32, i32, f64) {
  let mut sum = 0;
  let mut max = 0;
  let mut min = 0;
  let mut counter = 0;

  for i in arr {
    counter += 1;
    sum += i;

    if max > i {
      max = i
    } else if min < i {
      min = i
    }
  }

  let average = sum as f64 / counter as f64;
  return (sum, max, min, average);
}

#[test]
fn test_array_stats() {
  let array: [i32; 5] = [1, -2, 3, 4, 5];
  let result = array_stats(array);
  println!("result {:?}", result)
}

// Problem 2: String Character Analysis (Medium)
/**
 * Description: Given a String, count:
 * Total number of characters
 * Number of alphabetic characters (a-z, A-Z)
 * Number of digit characters (0-9)
 * Number of whitespace characters (space, tab, newline)
 *
 * fn analyze_string(s: String) -> (usize, usize, usize, usize)
 * .is_alphabetic()
 * .is_digit(10)
 * .is_whitespace()
 */
fn analyze_string(s: &String) -> (usize, usize, usize, usize) {
  let total_chars = s.len();
  let mut number_alphabetic = 0;
  let mut number_digit = 0;
  let mut number_whitespace = 0;
  let chars = s.chars();
  for value in chars {
    if value.is_alphabetic() {
      number_alphabetic += 1;
    }

    if value.is_whitespace() {
      number_whitespace += 1;
    }

    if value.is_digit(10) {
      number_digit += 1;
    }
  }

  (
    total_chars,
    number_alphabetic,
    number_digit,
    number_whitespace,
  )
}

#[test]
fn test_analyze_string() {
  let hello = String::from("HELO helo guys 321 ");
  let result = analyze_string(&hello);

  println!("{:?}", result);
}

// Problem 3: Tuple Matrix Operations (Medium-Hard)
/**
 * Description:
 * Given two 2D arrays (3x3 matrices) represented as tuples of tuples, perform element-wise operations:
 *
 * Add the two matrices
 * Subtract the second from the first
 * Multiply each element by a scalar

fn matrix_operations(
    a: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    b: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    scalar: i32
) -> (
    ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
    ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
)

    Input:
a: First 3x3 matrix as tuple of tuples
b: Second 3x3 matrix as tuple of tuples
scalar: Integer to multiply with

Output:
Return a tuple of three matrices:
Matrix addition: a + b (element-wise)
Matrix subtraction: a - b (element-wise)
Scalar multiplication: a × scalar
 */

fn matrix_operations(
  a: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  b: ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  scalar: i32,
) -> (
  ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
  ((i32, i32, i32), (i32, i32, i32), (i32, i32, i32)),
) {
  let addition = (
    (a.0.0 + b.0.0, a.0.1 + b.0.1, a.0.2 + b.0.2),
    (a.1.0 + b.1.0, a.1.1 + b.1.1, a.1.2 + b.1.2),
    (a.2.0 + b.2.0, a.2.1 + b.2.1, a.2.2 + b.2.2),
  );
  let subtraction = (
    (a.0.0 - b.0.0, a.0.1 - b.0.1, a.0.2 - b.0.2),
    (a.1.0 - b.1.0, a.1.1 - b.1.1, a.1.2 - b.1.2),
    (a.2.0 - b.2.0, a.2.1 - b.2.1, a.2.2 - b.2.2),
  );
  let scalar_calculated = (
    (a.0.0 * scalar, a.0.1 * scalar, a.0.2 * scalar),
    (a.1.0 * scalar, a.1.1 * scalar, a.1.2 * scalar),
    (a.2.0 * scalar, a.2.1 * scalar, a.2.2 * scalar),
  );

  // println!("a {:?}", a);
  // println!("b {:?}", b);
  // println!("a.0 {:?}",a.0);
  // println!("a.0.0 {:?}",a.0.0);
  // println!("a.0.1 {:?}",a.0.1);
  // println!("scalar {}", scalar);
  (addition, subtraction, scalar_calculated)
}

#[test]
fn test_matrix_operations() {
  let a = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
  let b = ((9, 8, 7), (6, 5, 4), (3, 2, 1));
  let c = 4;

  let result = matrix_operations(a, b, c);
  println!("result {:?}", result)
}

/**
 * Student Grade Management (Intermediate)
 * Description:
 * Create a system to manage student grades using structs, enums, and collections. A student can have multiple courses with grades represented as either:
 * Numeric score (0-100)
 * Letter grade (A, B, C, D, F)
 *
 * Requirements:
 * Define appropriate structs and enums
 * Add courses to a student
 * Calculate average grade
 * Find highest and lowest grade
 * Convert all grades to a consistent format
 *
 * Input/Output:
 *  Create a student, add courses with mixed grade types
 * Calculate statistics
 * Convert grades to numeric format (A=90, B=80, C=70, D=60, F=50)
 *
 * Constraints:
 * Use HashMap for course list (course_name -> Grade)
 * Handle empty grade list appropriately (return None)
 * Use pattern matching for grade conversions
 * Demonstrate borrowing (&mut self, &self) correctly
 *
 * Edge cases:
 * Empty course list
 * Invalid letter grades (should be handled gracefully)
 * Duplicate course names (should update grade)
 *
 * let mut student = Student::new(String::from("Alice"));
student.add_course(String::from("Math"), Grade::Numeric(85));
student.add_course(String::from("Science"), Grade::Letter('B'));
assert_eq!(student.average_grade(), Some(82.5));
student.convert_all_to_numeric();

 * Note : structs, enums, HashMap, methods, pattern matching, and borrowing.
 *
 */
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Grade {
  Numeric(u32),
  Letter(char),
}

impl Grade {
  fn to_numeric(&self) -> u32 {
    match self {
      Grade::Letter('A') => 90,
      Grade::Letter('B') => 80,
      Grade::Letter('C') => 70,
      Grade::Letter('D') => 60,
      Grade::Letter('E') => 50,
      Grade::Numeric(num) => *num,
      _ => 0,
    }
  }
}

#[derive(Debug, Clone)]
struct Student {
  name: String,
  courses: HashMap<String, Grade>,
}

// #[derive(Display)]
impl Student {
  fn new(name: String) -> Self {
    Student {
      name,
      courses: HashMap::new(),
    }
  }
  fn add_course(&mut self, course_name: String, grade: Grade) {
    self.courses.insert(course_name, grade);
  }
  fn average_grade(&self) -> Option<f64> {
    let mut amount: u32 = 0;
    for course in self.courses.values() {
      amount += course.to_numeric();
    }
    if amount == 0 {
      None
    } else {
      Some(amount as f64 / self.courses.len() as f64)
    }
  }
  fn highest_grade(&self) -> Option<i32> {
    let mut highest = 0;
    for (_, val) in self.courses.iter() {
      if val.to_numeric() > highest {
        highest = val.to_numeric();
      }
    }

    if highest == 0 {
      None
    } else {
      Some(highest as i32)
    }
  }
  fn lowest_grade(&self) -> Option<i32> {
    let mut lowest = 100;
    for (_, val) in self.courses.iter() {
      if val.to_numeric() < lowest {
        lowest = val.to_numeric();
      }
    }

    if lowest == 0 {
      None
    } else {
      Some(lowest as i32)
    }
  }
  fn convert_all_to_numeric(&mut self) {
    for (key, _) in self.courses.iter() {
      if let Some(x) = self.courses.get(key) {
        x.to_numeric();
      }
    }
  }
}

#[test]
fn test_problem4() {
  let mut student = Student::new(String::from("Alice"));
  student.add_course(String::from("Math"), Grade::Numeric(85));
  student.add_course(String::from("Science"), Grade::Letter('B'));
  assert_eq!(student.average_grade(), Some(82.5));
  student.convert_all_to_numeric();

  // println!("Student {:?}", student);
  // println!("average_grade {:?}", student.average_grade());
  // println!("highest {:?}", student.highest_grade());
  // println!("lowest {:?}", student.lowest_grade());
  // println!("convert {:?}", student.convert_all_to_numeric());
}

/**
 * Description :
 * Problem 5: Inventory System with Lifetimes (Advanced)
 * Create an inventory management system with lifetime annotations. Items have varying lifespans (perishable vs non-perishable), and we need to track items that are about to expire.
 *
 * Requirements :
 * define structs with explicit lifetime annotations
 * implement methods that return reference with appropriate lifetimes
 * filter items based on remaining shelf life
 * handle mutable updates to iventory
 *
 * Function Signature
 struct Item<'a> {
    name: &'a str,
    days_until_expiry: u32,
    category: &'a str,
}

struct Inventory<'a> {
    items: Vec<Item<'a>>,
    warehouse_name: String,
}

impl<'a> Inventory<'a> {
    fn new(warehouse_name: String) -> Self { /* implement */ }

    fn add_item(&mut self, name: &'a str, days_until_expiry: u32, category: &'a str) { /* implement */ }

    // Return reference with same lifetime as self
    fn find_by_name(&self, name: &str) -> Option<&Item<'a>> { /* implement */ }

    // Return mutable reference
    fn update_expiry(&mut self, name: &str, new_days: u32) -> bool { /* implement */ }

    // Return vector of references to items expiring soon
    fn get_expiring_soon(&self, threshold: u32) -> Vec<&Item<'a>> { /* implement */ }

    // Calculate average days until expiry for a category
    fn avg_expiry_by_category(&self, category: &str) -> Option<f64> { /* implement */ }
}
 * Constraints :
 * Use explicit lifetime annotations everywhere needed
 * demonstrate understanding of lifetime elision vs explicit annotations
 * use iterator methis (not manual loops)
 * handle cases where no items match criteria (return None/ empty collections)
 *
 * Example :
 let mut inventory = Inventory::new(String::from("Main Warehouse"));
inventory.add_item("Milk", 5, "Dairy");
inventory.add_item("Cheese", 30, "Dairy");
inventory.add_item("Rice", 365, "Grains");

assert_eq!(inventory.find_by_name("Milk").unwrap().days_until_expiry, 5);
assert!(inventory.update_expiry("Milk", 3));
let expiring = inventory.get_expiring_soon(7); // Returns Milk (3 days)
assert_eq!(inventory.avg_expiry_by_category("Dairy"), Some(16.5));
 *
 * Edge cases :
 * Empty inventory
 * Multiple items with same name
 * Category with no items
 * Zero or negative expiry days
 */
struct Item<'a> {
  name: &'a str,
  days_until_expiry: u32,
  category: &'a str,
}

struct Inventory<'a> {
  items: Vec<Item<'a>>,
  warehouse_name: String,
}

impl<'a> Inventory<'a> {
  fn new(warehouse_name: String) -> Self {
    Inventory {
      items: Vec::new(),
      warehouse_name,
    }
  }

  fn add_item(&mut self, name: &'a str, days_until_expiry: u32, category: &'a str) {
    self.items.push(Item {
      name,
      days_until_expiry,
      category,
    });
  }

  // Return reference with same lifetime as self
  fn find_by_name(&self, name: &str) -> Option<&Item<'a>> {
    let found = self
      .items
      .iter()
      .find(|e| e.name.eq_ignore_ascii_case(name));
    if let Some(item) = found {
      Some(item)
    } else {
      None
    }
  }

  // Return mutable reference
  fn update_expiry(&mut self, name: &str, new_days: u32) -> bool {
    if let Some(val) = self
      .items
      .iter_mut()
      .find(|e| e.name.eq_ignore_ascii_case(name))
    {
      val.days_until_expiry = new_days;
      true;
    }
    false
  }

  // Return vector of references to items expiring soon
  fn get_expiring_soon(&self, threshold: u32) -> Vec<&Item<'a>> {
    self
      .items
      .iter()
      .filter(|val| val.days_until_expiry <= threshold)
      .collect()
  }

  // Calculate average days until expiry for a category
  fn avg_expiry_by_category(&self, category: &str) -> Option<f64> {
    let found: Vec<u32> = self
      .items
      .iter()
      .filter(|val| val.category.eq_ignore_ascii_case(category))
      .map(|val| val.days_until_expiry)
      .collect();

    if found.is_empty() {
      None
    } else {
      let total: u32 = found.iter().sum();
      Some(total as f64 / found.len() as f64)
    }
  }
}

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

struct CacheEntry<V> {
  value: V,
  access_count: u32,
}

struct LruCache<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  capacity: usize,
  store: HashMap<K, Rc<RefCell<CacheEntry<V>>>>,
}

impl<K, V> LruCache<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  fn new(capacity: usize) -> Self {}
  fn get (&mut self, key) -> Option<V> {}
  fn insert (&mut self, key : K, value : V){}
  fn remove (&must self, key &K) -> Option<V>{}
  fn clear (&mut self) {}
  fn len (&self)->usize {}
  fn most_frequent (&self) -> Option<K, V>{}
}
