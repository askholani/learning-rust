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
pub struct Item<'a> {
  pub name: &'a str,
  pub days_until_expiry: u32,
  pub category: &'a str,
}

pub struct Inventory<'a> {
  pub items: Vec<Item<'a>>,
  pub warehouse_name: String,
}

impl<'a> Inventory<'a> {
  pub fn new(warehouse_name: String) -> Self {
    Inventory {
      items: Vec::new(),
      warehouse_name,
    }
  }

  pub fn add_item(&mut self, name: &'a str, days_until_expiry: u32, category: &'a str) {
    self.items.push(Item {
      name,
      days_until_expiry,
      category,
    });
  }

  // Return reference with same lifetime as self
  pub fn find_by_name(&self, name: &str) -> Option<&Item<'a>> {
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
  pub fn update_expiry(&mut self, name: &str, new_days: u32) -> bool {
    if let Some(val) = self
      .items
      .iter_mut()
      .find(|e| e.name.eq_ignore_ascii_case(name))
    {
      val.days_until_expiry = new_days;
      return true;
    }
    false
  }

  // Return vector of references to items expiring soon
  pub fn get_expiring_soon(&self, threshold: u32) -> Vec<&Item<'a>> {
    self
      .items
      .iter()
      .filter(|val| val.days_until_expiry <= threshold)
      .collect()
  }

  // Calculate average days until expiry for a category
  pub fn avg_expiry_by_category(&self, category: &str) -> Option<f64> {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_inventory() {
    let mut inventory = Inventory::new(String::from("Main Warehouse"));
    inventory.add_item("Milk", 5, "Dairy");
    inventory.add_item("Cheese", 30, "Dairy");
    inventory.add_item("Rice", 365, "Grains");

    assert_eq!(inventory.find_by_name("Milk").unwrap().days_until_expiry, 5);
    assert!(inventory.update_expiry("Milk", 3));
    let expiring = inventory.get_expiring_soon(7);
    assert_eq!(expiring.len(), 1);
    assert_eq!(inventory.avg_expiry_by_category("Dairy"), Some(16.5));
  }
}
