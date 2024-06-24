use std::cell::RefCell;
use std::collections::binary_heap::IntoIter;
use std::collections::BinaryHeap;

use crate::structs::order::Order;

#[derive(Debug, Clone)]
pub struct ModifiableBinaryHeap<T: Clone + Ord> {
    heap: RefCell<BinaryHeap<T>>,
}

unsafe impl Sync for ModifiableBinaryHeap<Order> {}

impl<T: Clone + Ord> ModifiableBinaryHeap<T> {
    // Constructor to create a new empty heap
    pub fn new() -> Self {
        ModifiableBinaryHeap {
            heap: RefCell::new(BinaryHeap::new()),
        }
    }

    // Method to push an element onto the heap
    pub fn push(&self, item: T) {
        self.heap.borrow_mut().push(item);
    }

    // Method to peek at the top element of the heap
    pub fn peek(&self) -> Option<T> {
        self.heap.borrow().peek().cloned()
    }

    // Method to retain elements based on a closure
    pub fn retain<F>(&self, retain_fn: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.heap.borrow_mut().retain(retain_fn);
    }

    // Method to pop the top element from the heap
    pub fn pop(&self) -> Option<T> {
        self.heap.borrow_mut().pop()
    }

    // Method to check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.heap.borrow().is_empty()
    }

    // Method to iterate over the heap (not ordered)
    pub fn iter(&self) -> IntoIter<T> {
        self.heap.borrow().clone().into_iter()
    }

    // Method to iterate over the heap in sorted order
    pub fn iter_sorted(&self) -> Vec<T> {
        let mut heap_borrow = self.heap.borrow_mut().clone();
        let mut heap_vec: Vec<_> = heap_borrow.drain().collect();
        heap_vec.sort();
        heap_vec
    }

    // Method to get the length of the heap
    pub fn len(&self) -> usize {
        self.heap.borrow().len()
    }

    // Method to convert the heap into a vector
    pub fn into_vec(&self) -> Vec<T> {
        self.heap.clone().into_inner().into_vec()
    }

    // Method to modify an element (example implementation)
    // This is just a stub function; it doesn't do anything meaningful without specific requirements.
    pub fn modify<F>(&self, mut modify_fn: F)
    where
        F: FnMut(&mut T),
    {
        let mut heap_borrow = self.heap.borrow_mut();
        let mut heap_vec: Vec<_> = heap_borrow.drain().collect();

        for item in &mut heap_vec {
            modify_fn(item);
        }

        // Rebuild the heap after modification
        heap_vec.sort(); // This sort is needed to maintain the heap property.
        *heap_borrow = BinaryHeap::from(heap_vec);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use ulid::Ulid;

    use super::*;
    use crate::{
        enums::{order_type::OrderType, side::OrderSide},
        structs::order::Order,
    };
    use std::time::Instant;

    #[test]
    fn test_modifiable_binary_heap() {
        let heap = ModifiableBinaryHeap::new();
        let order1 = Order {
            id: Ulid::new().into(),
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Buy,
            quantity: 1.0,
            non_mut_quantity: 1.0,
            price: Some(1.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        let order2 = Order {
            id: Ulid::new().into(),
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Buy,
            quantity: 2.0,
            non_mut_quantity: 2.0,
            price: Some(2.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        let id = Ulid::new().into();
        let order3 = Order {
            id,
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Buy,
            quantity: 3.0,
            non_mut_quantity: 3.0,
            price: Some(3.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        heap.push(order3.clone());
        heap.push(order2.clone());
        heap.push(order1.clone());

        assert_eq!(heap.len(), 3);

        heap.modify(|item| {
            if item.id == id {
                item.quantity = 45.0;
            }
        });

        assert_eq!(heap.len(), 3);
        let modified_order = heap.peek().unwrap();
        assert_eq!(modified_order.quantity, 45.0);
    }

    #[test]
    fn test_sell_modifiable_binary_heap() {
        let heap = ModifiableBinaryHeap::new();
        let order1 = Order {
            id: Ulid::new().into(),
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Sell,
            quantity: 1.0,
            non_mut_quantity: 1.0,
            price: Some(1.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        let order2 = Order {
            id: Ulid::new().into(),
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Sell,
            quantity: 2.0,
            non_mut_quantity: 2.0,
            price: Some(2.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        let id = Ulid::new().into();
        let order3 = Order {
            id,
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Sell,
            quantity: 3.0,
            non_mut_quantity: 3.0,
            price: Some(3.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        heap.push(order3.clone());
        heap.push(order2.clone());
        heap.push(order1.clone());

        assert_eq!(heap.len(), 3);

        heap.modify(|item| {
            if item.id == id {
                item.quantity = 45.0;
            }
        });

        assert_eq!(heap.len(), 3);
        let modified_order = heap.peek().unwrap();
        assert_eq!(modified_order.quantity, 1.0);
    }

    #[test]
    fn test_reversed_modifiable_binary_heap() {
        let heap = ModifiableBinaryHeap::new();
        let order1 = Order {
            id: Ulid::new().into(),
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Buy,
            quantity: 1.0,
            non_mut_quantity: 1.0,
            price: Some(1.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        let order2 = Order {
            id: Ulid::new().into(),
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Buy,
            quantity: 2.0,
            non_mut_quantity: 2.0,
            price: Some(2.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        let id = Ulid::new().into();
        let order3 = Order {
            id,
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Buy,
            quantity: 3.0,
            non_mut_quantity: 3.0,
            price: Some(3.0),
            order_type: OrderType::Limit,
            status: Default::default(),
            payment_status: Default::default(),
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        };
        heap.push(Reverse(order3.clone()));
        heap.push(Reverse(order2.clone()));
        heap.push(Reverse(order1.clone()));

        assert_eq!(heap.len(), 3);

        heap.modify(|item| {
            if item.0.id == id {
                item.0.quantity = 45.0;
            }
        });

        assert_eq!(heap.len(), 3);
        let modified_order = heap.peek().unwrap();
        assert_eq!(modified_order.0.quantity, 1.0);
    }
}
