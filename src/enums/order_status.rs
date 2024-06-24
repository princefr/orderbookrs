use async_graphql::*;
use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy, Enum)]
pub enum OrderStatus {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,
    #[serde(rename = "FILLED")]
    Filled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Open
    }
}

impl OrderStatus {
    pub fn to_string(&self) -> String {
        match self {
            OrderStatus::Open => "Open".to_string(),
            OrderStatus::Closed => "Closed".to_string(),
            OrderStatus::Cancelled => "Cancelled".to_string(),
            OrderStatus::Pending => "Pending".to_string(),
            OrderStatus::PartiallyFilled => "PartiallyFilled".to_string(),
            OrderStatus::Filled => "Filled".to_string(),
        }
    }

    pub fn from_string(s: &str) -> OrderStatus {
        match s {
            "Open" => OrderStatus::Open,
            "Closed" => OrderStatus::Closed,
            "Cancelled" => OrderStatus::Cancelled,
            "Pending" => OrderStatus::Pending,
            "PartiallyFilled" => OrderStatus::PartiallyFilled,
            "Filled" => OrderStatus::Filled,
            _ => OrderStatus::Open,
        }
    }
}

impl Eq for OrderStatus {}

impl Into<i32> for OrderStatus {
    fn into(self) -> i32 {
        match self {
            OrderStatus::Open => 0,
            OrderStatus::Closed => 1,
            OrderStatus::Cancelled => 2,
            OrderStatus::Pending => 3,
            OrderStatus::PartiallyFilled => 4,
            OrderStatus::Filled => 5,
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderStatus::Open => write!(f, "Open"),
            OrderStatus::Closed => write!(f, "Closed"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::PartiallyFilled => write!(f, "PartiallyFilled"),
            OrderStatus::Filled => write!(f, "Filled"),
        }
    }
}

