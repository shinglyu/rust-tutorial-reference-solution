#![allow(dead_code)]

////////////////////////////////////////////////////////////////////////////////
// The trait
////////////////////////////////////////////////////////////////////////////////

trait Price {
    fn price(&self, item_name: &str) -> Option<f32>;

    fn total_price(&self, shopping_list: &[&str]) -> Option<f32> {
        // Goal: compute the total price of all items in the shopping
        // list. If any of the options are not present, return `None`.
        None
    }
}

////////////////////////////////////////////////////////////////////////////////
// Store
////////////////////////////////////////////////////////////////////////////////

struct Store {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    name: &'static str,
    price: f32,
}

impl Store {
    fn new(name: String) -> Store {
        Store {
            name: name,
            items: vec![],
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

impl Price for Store {
    fn price(&self, item_name: &str) -> Option<f32> {
        for item in &self.items {
            if item.name == item_name {
                return Some(item.price);
            }
        }
        None
    }
}

fn build_store() -> Store {
    let mut store = Store::new(format!("Rustmart"));
    store.add_item(Item { name: "chocolate", price: 5.0 });
    store.add_item(Item { name: "socks", price: 23.0 });
    store.add_item(Item { name: "plush Mozilla dinosaur", price: 13.0 });
    store
}

////////////////////////////////////////////////////////////////////////////////
// Factory
////////////////////////////////////////////////////////////////////////////////

// A factory for just a single kind of item
struct Factory {
    item_name: &'static str,
    wholesale_price: f32,
}

impl Price for Factory {
    fn price(&self, item_name: &str) -> Option<f32> {
        if self.item_name == item_name {
            Some(self.wholesale_price)
        } else {
            None
        }
    }
}

fn build_factory() -> Factory {
    Factory {
        item_name: "sprocket",
        wholesale_price: 7.67,
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[test]
fn total_price_store() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur"];
    assert_eq!(store.total_price(&list), Some(18.0));
}

#[test]
fn total_price_missing_store() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur", "fork and knife"];
    assert_eq!(store.total_price(&list), None);
}

#[test]
fn total_price_factory() {
    let factory = build_factory();
    let list = vec!["sprocket"];
    assert_eq!(factory.total_price(&list), Some(7.67));
}

#[test]
fn total_price_missing_factory() {
    let factory = build_factory();
    let list = vec!["sprocket", "socks"];
    assert_eq!(factory.total_price(&list), None);
}

