use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum OrderbookUpdateType {
    /*Trigger saving of the new order with Pending Status */
    New,
    /*Trigger saving of the new order with Open Status */
    Place,
    /*Trigger saving of the new order with Cancelled Status */
    Cancel,
    /*Trigger saving of the new order with Partially Filled Status */
    Update,
    /*Trigger saving of the new order with Filled Status */
    NewTrades,
    Filled,
}

impl fmt::Display for OrderbookUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderbookUpdateType::New => write!(f, "New"),
            OrderbookUpdateType::Place => write!(f, "Place"),
            OrderbookUpdateType::Cancel => write!(f, "Cancel"),
            OrderbookUpdateType::Update => write!(f, "Update"),
            OrderbookUpdateType::NewTrades => write!(f, "NewTrades"),
            OrderbookUpdateType::Filled => write!(f, "Filled"),
        }
    }
}

impl Into<i32> for OrderbookUpdateType {
    fn into(self) -> i32 {
        match self {
            OrderbookUpdateType::New => 0,
            OrderbookUpdateType::Place => 1,
            OrderbookUpdateType::Cancel => 2,
            OrderbookUpdateType::Update => 3,
            OrderbookUpdateType::NewTrades => 4,
            OrderbookUpdateType::Filled => 5,
        }
    }
}

impl Default for OrderbookUpdateType {
    fn default() -> Self {
        OrderbookUpdateType::New
    }
}
