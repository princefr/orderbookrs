
use serde::{Deserialize, Serialize};


use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum TradeStatus {
    Swapped,
    Pending,
    Failed,
}

impl Into<i32> for TradeStatus {
    fn into(self) -> i32 {
        match self {
            TradeStatus::Swapped => 0,
            TradeStatus::Pending => 1,
            TradeStatus::Failed => 2,
        }
    }
}

impl Default for TradeStatus {
    fn default() -> Self {
        TradeStatus::Pending
    }
}

impl TradeStatus {
    pub fn from_string(s: &str) -> TradeStatus {
        match s {
            "Swapped" => TradeStatus::Swapped,
            "Pending" => TradeStatus::Pending,
            "Failed" => TradeStatus::Failed,
            _ => TradeStatus::Failed,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TradeStatus::Swapped => "Swapped".to_string(),
            TradeStatus::Pending => "Pending".to_string(),
            TradeStatus::Failed => "Failed".to_string(),
        }
    }
}

impl fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeStatus::Swapped => write!(f, "Swapped"),
            TradeStatus::Pending => write!(f, "Pending"),
            TradeStatus::Failed => write!(f, "Failed"),
        }
    }
}
