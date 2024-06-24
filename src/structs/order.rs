use crate::enums::payment_status::PaymentStatus;
use crate::enums::side::OrderSide;
use crate::enums::{order_status::OrderStatus, order_type::OrderType};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use std::time::Instant;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Order {
    pub id: u128,
    #[serde(rename = "userId")]
    pub user_id: u128,
    pub symbol: u128,
    pub side: OrderSide,
    pub quantity: f64,
    #[serde(rename = "nonMutQuantity")]
    pub non_mut_quantity: f64,
    pub price: Option<f64>, // None for market orders
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    pub status: OrderStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status: PaymentStatus,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

impl Order {

    #[cfg(test)]
    pub fn get_test_order<'a>(symbol: u128, user_id: u128) -> Order {
        Order {
            id: Ulid::new().into(),
            symbol,
            user_id,
            side: OrderSide::Buy,
            price: Some(100.0),
            quantity: 100.0,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            non_mut_quantity: 100.0,
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
            payment_status: Default::default(),
        }
    }
}

impl Eq for Order {}

impl Default for Order {
    fn default() -> Self {
        Order {
            id: Ulid::new().into(),
            user_id: Ulid::new().into(),
            symbol: Ulid::new().into(),
            side: OrderSide::Buy,
            quantity: 0.0,
            price: None,
            non_mut_quantity: 0.0,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            payment_status: PaymentStatus::Pending,
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
        }
    }
}

impl Order {
    pub fn new(
        user_id: u128,
        symbol: u128,
        side: OrderSide,
        quantity: f64,
        price: Option<f64>,
        order_type: OrderType,
    ) -> Order {
        Order {
            id: Ulid::new().into(),
            user_id,
            symbol,
            side,
            quantity,
            price,
            order_type,
            non_mut_quantity: quantity,
            created_at: Instant::now().elapsed().as_secs(),
            updated_at: Instant::now().elapsed().as_secs(),
            status: Default::default(),
            payment_status: Default::default(),
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.side {
            OrderSide::Buy => self.price.partial_cmp(&other.price),
            OrderSide::Sell => other.price.partial_cmp(&self.price),
        }
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.side {
            OrderSide::Buy => self.price.partial_cmp(&other.price).unwrap(),
            OrderSide::Sell => other.price.partial_cmp(&self.price).unwrap(),
        }
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Order: {} {} {} {} {} {}",
            self.id,
            self.user_id,
            self.symbol,
            self.side,
            self.quantity,
            self.price.unwrap_or(0.0)
        )
    }
}
