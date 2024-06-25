use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Cancelled,
    Refunded,
    Unknown,
}

impl Into<i32> for PaymentStatus {
    fn into(self) -> i32 {
        match self {
            PaymentStatus::Pending => 0,
            PaymentStatus::Paid => 1,
            PaymentStatus::Failed => 2,
            PaymentStatus::Cancelled => 3,
            PaymentStatus::Refunded => 4,
            PaymentStatus::Unknown => 5,
        }
    }
}

impl Default for PaymentStatus {
    fn default() -> Self {
        PaymentStatus::Pending
    }
}

impl PaymentStatus {
    pub fn from_string(s: &str) -> PaymentStatus {
        match s {
            "Pending" => PaymentStatus::Pending,
            "Paid" => PaymentStatus::Paid,
            "Failed" => PaymentStatus::Failed,
            "Cancelled" => PaymentStatus::Cancelled,
            "Refunded" => PaymentStatus::Refunded,
            _ => PaymentStatus::Unknown,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PaymentStatus::Pending => "Pending".to_string(),
            PaymentStatus::Paid => "Paid".to_string(),
            PaymentStatus::Failed => "Failed".to_string(),
            PaymentStatus::Cancelled => "Cancelled".to_string(),
            PaymentStatus::Refunded => "Refunded".to_string(),
            PaymentStatus::Unknown => "Unknown".to_string(),
        }
    }
}

impl fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentStatus::Pending => write!(f, "Pending"),
            PaymentStatus::Paid => write!(f, "Paid"),
            PaymentStatus::Failed => write!(f, "Failed"),
            PaymentStatus::Cancelled => write!(f, "Cancelled"),
            PaymentStatus::Refunded => write!(f, "Refunded"),
            PaymentStatus::Unknown => write!(f, "Unknown"),
        }
    }
}
