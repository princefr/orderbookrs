
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use std::time::Instant;

use crate::enums::trade_status::TradeStatus;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Option<u128>,
    pub buy_order_id: u128,
    pub sell_order_id: u128,
    pub buy_user_id: u128,
    pub sell_user_id: u128,
    pub price: f64,
    pub quantity: f64,
    pub status: TradeStatus,
    pub symbol: u128,
    pub created_at: Option<u64>,
    pub updated_at: Option<u64>,
}

impl Trade {

    /// Generate a Trade with 10 in price and 2 in quantity for testing purpose
    /// #Parameters
    /// * `symbol` - The symbol of the trade
    /// * `buy_order_id` - The order_id of the buy order
    /// * `sell_order_id` - The order_id of the sell order
    pub fn get_trade_10_2(
        symbol: u128,
        buy_order_id: u128,
        sell_order_id: u128,
        buy_user_id: u128,
        sell_user_id: u128,
    ) -> Trade {
        Trade {
            id: Some(Ulid::new().into()),
            symbol,
            price: 10.0,
            quantity: 2.0,
            created_at: Some(Instant::now().elapsed().as_secs()),
            updated_at: Some(Instant::now().elapsed().as_secs()),
            status: Default::default(),
            buy_order_id,
            sell_order_id,
            buy_user_id,
            sell_user_id,
        }
    }

    pub fn get_trade_10_5(
        symbol: u128,
        buy_order_id: u128,
        sell_order_id: u128,
        buy_user_id: u128,
        sell_user_id: u128,
    ) -> Trade {
        Trade {
            id: Some(Ulid::new().into()),
            symbol,
            price: 10.0,
            quantity: 5.0,
            created_at: Some(Instant::now().elapsed().as_secs()),
            updated_at: Some(Instant::now().elapsed().as_secs()),
            status: Default::default(),
            buy_order_id,
            sell_order_id,
            buy_user_id,
            sell_user_id,
        }
    }

    pub fn get_trade_15_2(
        symbol: u128,
        buy_order_id: u128,
        sell_order_id: u128,
        buy_user_id: u128,
        sell_user_id: u128,
    ) -> Trade {
        Trade {
            id: Some(Ulid::new().into()),
            symbol,
            price: 15.0,
            quantity: 2.0,
            created_at: Some(Instant::now().elapsed().as_secs()),
            updated_at: Some(Instant::now().elapsed().as_secs()),
            status: Default::default(),
            buy_order_id,
            sell_order_id,
            buy_user_id,
            sell_user_id,
        }
    }
}

impl Default for Trade {
    fn default() -> Self {
        Trade {
            id: None,
            buy_order_id: Ulid::new().into(),
            sell_order_id: Ulid::new().into(),
            buy_user_id: Ulid::new().into(),
            sell_user_id: Ulid::new().into(),
            price: 0.0,
            quantity: 0.0,
            status: Default::default(),
            symbol: Ulid::new().into(),
            created_at: Some(Instant::now().elapsed().as_secs()),
            updated_at: Some(Instant::now().elapsed().as_secs()),
        }
    }
}
