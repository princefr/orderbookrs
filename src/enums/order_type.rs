use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
}

impl Eq for OrderType {}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

impl Into<i32> for OrderType {
    fn into(self) -> i32 {
        match self {
            OrderType::Limit => 0,
            OrderType::Market => 1,
        }
    }
}

impl OrderType {
    pub fn to_string(&self) -> String {
        match self {
            OrderType::Market => "MARKET".to_string(),
            OrderType::Limit => "LIMIT".to_string(),
        }
    }

    pub fn from_string(s: &str) -> OrderType {
        match s {
            "MARKET" => OrderType::Market,
            "LIMIT" => OrderType::Limit,
            _ => OrderType::Limit,
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::Market => write!(f, "MARKET"),
        }
    }
}
