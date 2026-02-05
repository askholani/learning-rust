/**
 * LRUCache :
1. capacity : number of node inside it
2. nodes : storing cache data in linked list structure
3. map : pair key and index
4. head : first data in nodes
5. tail : last data in a node
6. free : index for a node removed

 *
* Description :
* Problem 7: Optimized LRU Cache with 0(1) Operations (Alogrithm Optimization)
* Implement a true 0(1) LRU cache using a combination of HashMap and a dobuly-linked list pattern (simulated with Vec Indices).
*
* Requirements :
* All operations must be 0(1) average case
* No linear scans through any collection
* Properly handle edge cases
* Use interior mutability only where necessary

* Instead of VecDeque, use a HashMap storing indices into a Vec that maintains nodes in access order. Each node contains :
* Key
* Value
* Previous Index
* Next Index

* Function Signature :
use std::collections::HashMap;
use std::hash::Hash;

struct LRUCache <K, V>
where
K : Eq + Hash + Clone, {
capaticy : usize,
nodes : Vec<Node<K, V>>,
map : HashMap<K, usize>, key -> index in ndoes
head : Option<usize>, index of most recently used
tail : Option<usize>, index of least recently used
free : Vec<usize>,
}

struct Node<K,V> {
key : K,
value : V,
prev : Option<usize>,
next : Option<usize>
}

impl <K, V> LRUCache <K,V>
where
K : Eq + Hash + Clone,
{
fn new (capacity: uszie) -> Self {}

fn get (&mut self, key : &K) -> Option <&V> {}

fn get_mut (&mut self, key : &K) -> Option<&mut V> {}

fn insert (&mut self, key : K, value : V) -> Option <V> {}

fn remove (&mut self, key : &K) -> Option<V> {}

fn len (&self) -> usize {}

fn is_empty (&self) -> bool {}

}
Iterator over entries in LRU order (most to least recent)
fn iter (&self) -> impl Iteraotr <Item = (&K, &V)> {}

constraints :
get () and insert must be O(1) average
must handle capacity = 0 correctly
Return references where appropriate (not clones)
No linear scans in any operation

Example:
let mut cache = LRUCache::new(2);
cache.insert("a",1);
cache.insert("b",2);
assert_eq!(cache.get(&"a"), Some(&1));
cache.insert("c",3); // evicts "b"
assert_eq!(cache.get(&"b"), None);
*/
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

struct LRUCache<K, V>
where
  K: Eq + Hash + Clone,
  V: Default,
{
  capacity: usize,
  nodes: Vec<Node<K, V>>,
  map: HashMap<K, usize>, // key -> index in ndoes
  head: Option<usize>,    // index of most recently used
  tail: Option<usize>,    // index of least recently used
  free: Vec<usize>,       // indices of freed nodes for reuse
}

struct Node<K, V> {
  key: K,              // Needed so we can remove it from map during eviction
  value: V,            // Cached value
  prev: Option<usize>, // Index of the previous node (more recently used)
  next: Option<usize>, // Index of the next node (less recently used)
}

impl<K, V> LRUCache<K, V>
where
  K: Eq + Hash + Clone,
  V: Default,
{
  fn new(capacity: usize) -> Self {
    LRUCache {
      capacity,
      nodes: Vec::new(),
      map: HashMap::new(),
      head: None,
      tail: None,
      free: Vec::new(),
    }
  }

  fn get(&mut self, key: &K) -> Option<&V> {
    /*
    Return the value and mark the entry as most recently used.
    1.check if key exist in map, if not found -> return None
    2. retrieve the node index
    3. if node is already head, do nothing
    4. otherwise
        Detach node from its current position:
            Update its prev.next
            Update its next.prev
        if node was tail, update tail
    5. attach node at head :
        set its prev = None
        set its nex = old_head
        Update old head's prev
        Update head
    6. Return a reference to value
     */
    let index = *self.map.get(key)?;

    if self.head == Some(index) {
      return Some(&self.nodes[index].value);
    } else if self.tail == Some(index) {
      let node = self.nodes.get_mut(index)?;
      node.next = self.head;
      node.prev = None;
      self.head = Some(index);
      Some(&node.value)
    } else {
      let (prev_index, next_index) = {
        let node = &self.nodes[index];
        (node.prev, node.next)
      };

      if let Some(prev) = prev_index {
        self.nodes[prev].next = next_index;
      }

      if let Some(next) = next_index {
        self.nodes[next].prev = prev_index;
      }

      let old_head = self.head;
      {
        let node = &mut self.nodes[index];
        node.prev = None;
        node.next = old_head;
      }

      if let Some(h) = old_head {
        self.nodes[h].prev = Some(index);
      }

      self.head = Some(index);

      Some(&self.nodes[index].value)
    }
  }

  fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    /*
    Same as get, but return a mutable reference.
    1. Perform the exact same steps as get
    2. After moving node to head
    3. Return a mutable reference to value.
     */
    let index = *self.map.get(key)?;

    if self.head == Some(index) {
      return Some(&mut self.nodes[index].value);
    } else if self.tail == Some(index) {
      let node = self.nodes.get_mut(index)?;
      node.next = self.head;
      node.prev = None;
      self.head = Some(index);
      Some(&mut node.value)
    } else {
      let (prev_index, next_index) = {
        let node = &self.nodes[index];
        (node.prev, node.next)
      };

      if let Some(prev) = prev_index {
        self.nodes[prev].next = next_index;
      }

      if let Some(next) = next_index {
        self.nodes[next].prev = prev_index;
      }

      let old_head = self.head;
      {
        let node = &mut self.nodes[index];
        node.prev = None;
        node.next = old_head;
      }

      if let Some(h) = old_head {
        self.nodes[h].prev = Some(index);
      }

      self.head = Some(index);

      Some(&mut self.nodes[index].value)
    }
  }

  // fn insert(&mut self, key: K, value: V) -> Option<V> {
  //   /*
  //   Insert or update a value and maintain capacity
  //   Case A : key already exists
  //   1. Find index from map
  //   2. Replace node's value
  //   3. Move node to head
  //   4. Return Some(old_value)

  //   Case B : key does not exist
  //   1. Check capacity
  //       if cache is full :
  //       1. identify tail (LRU)
  //       2. Remove tail from linked list
  //       3. Remove tail key from map
  //       4. Save tail index into free
  //       5. Reduce active length
  //   2. Insert new node
  //       1. choose index
  //           Use one from free if available
  //           Otherwise append to nodes
  //       2. Create node
  //           prev = None
  //           next = current head
  //       3. Update current head's prev
  //       4. Update head
  //       5. if cache was emptyh, set tail = head
  //       6. insert (key -> index) into map
  //       7. increase active length
  //       8. Return None
  //    */
  //   if let Some(index) = self.map.get(&key) {
  //     let (prev_index, next_index) = {
  //       let node = &self.nodes[*index];
  //       (node.prev, node.next)
  //     };

  //     if let Some(prev) = prev_index {
  //       self.nodes[prev].next = next_index;
  //     }

  //     if let Some(next) = next_index {
  //       self.nodes[next].prev = prev_index;
  //     }

  //     let node = self.nodes.get_mut(*index)?;
  //     node.value = value;

  //     let old_node = self.nodes.get(self.head?)?;
  //     self.head = Some(*index);

  //     Some(old_node.value)
  //   } else {
  //     if self.nodes.len() >= self.capacity {
  //       let prev = {
  //         let node_tail = &self.nodes[self.tail?];
  //         self.map.remove(&node_tail.key);
  //         self.nodes[node_tail.prev?].next = None;
  //         node_tail.prev
  //       };
  //       self.free.push(prev?);
  //     }
  //     if self.free.len() > 0 {
  //       self.nodes[self.free[0]] = Node {
  //         key,
  //         value,
  //         prev: None,
  //         next: self.head,
  //       };
  //       self.head = Some(self.free[0]);
  //       self.map.insert(key.clone(), self.free[0]);
  //       self.free.pop();
  //       None
  //     } else {
  //       self.nodes.push(Node {
  //         key,
  //         value,
  //         prev: None,
  //         next: self.head,
  //       });
  //       self.head = Some(self.nodes.len());
  //       self.map.insert(key, self.nodes.len());
  //       None
  //     }
  //   }
  // }

  fn insert(&mut self, key: K, value: V) -> Option<V> {
    // === CASE A: key exists ===
    if let Some(&index) = self.map.get(&key) {
      // Detach node
      let (prev, next) = {
        let node = &self.nodes[index];
        (node.prev, node.next)
      };

      if let Some(p) = prev {
        self.nodes[p].next = next; // node in middle => B) A B C, so A.next not linked with B, but with C
      } else {
        self.head = next; // if the node in head
      }

      if let Some(n) = next {
        self.nodes[n].prev = prev; // node in middle => B) A B C, so C.prev not linked with B, bu with A
      } else {
        self.tail = prev; // if the node in tail
      }

      // Replace value
      let node = &mut self.nodes[index];
      let old = std::mem::replace(&mut node.value, value);

      // Move to head, node added automatically moving to head
      node.prev = None;
      node.next = self.head;

      if let Some(h) = self.head {
        self.nodes[h].prev = Some(index);
      }

      self.head = Some(index);
      if self.tail.is_none() {
        self.tail = Some(index);
      }

      return Some(old);
    }

    // === CASE B: key does not exist ===

    // Evict if full
    if self.map.len() == self.capacity {
      let tail_index = self.tail.expect("tail must exist");
      let (prev, key) = {
        let node = &self.nodes[tail_index];
        (node.prev, node.key.clone())
      };

      self.map.remove(&key);

      if let Some(p) = prev {
        self.nodes[p].next = None;
        self.tail = Some(p);
      } else {
        self.head = None;
        self.tail = None;
      }

      self.free.push(tail_index);
    }

    // Allocate index
    let index = if let Some(i) = self.free.pop() {
      self.nodes[i] = Node {
        key: key.clone(),
        value,
        prev: None,
        next: self.head,
      };
      i
    } else {
      let i = self.nodes.len();
      self.nodes.push(Node {
        key: key.clone(),
        value,
        prev: None,
        next: self.head,
      });
      i
    };

    // Fix head links
    if let Some(h) = self.head {
      self.nodes[h].prev = Some(index);
    }

    self.head = Some(index);
    if self.tail.is_none() {
      self.tail = Some(index);
    }

    self.map.insert(key, index);
    None
  }

  fn remove(&mut self, key: &K) -> Option<V> {
    /*
    Remove entry completely from cache.
    1. Look up index in map
        If not found -> return None
    2. Remove node from linked list
        Update prev.next
        Update next.prev
    3. if node is head, update head
    4. if node is tail, update tail
    5. remove key from map
    6. pust index into free
    7. decrease active length
    8. return removed value
     */

    if let Some(&index) = self.map.get(key) {
      // let node = self.nodes.get_mut(index)?;

      // let (prev, next, value) = {
      //   let node = self.nodes[index];
      //   (node.prev, node.next, node.value)
      // };

      let node = &mut self.nodes[index];

      let value = std::mem::take(&mut node.value);
      let prev = node.prev;
      let next = node.next;

      // https://chatgpt.com/c/697f3120-b520-8320-b139-046e1d6c4056
      if let Some(p) = prev {
        self.nodes[p].next = next
      } else {
        self.head = next
      }

      if let Some(n) = next {
        self.nodes[n].prev = prev
      } else {
        self.tail = prev
      }

      self.map.remove(key);
      self.free.push(index);

      Some(value);
    }

    None
  }

  fn len(&self) -> usize {
    /*
    Return number of active cache entries
    Return number of entries in map, if reflects active nodes only, nodes.len() includes freed nodes
     */
    self.map.len()
  }

  fn is_empty(&self) -> bool {
    /*
    is_empty() -> bool
    check if cache has no entries
    return true if len == 0, otherwise false
     */
    self.map.is_empty()
  }
}
