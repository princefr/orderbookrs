use super::{order::Order, trade::Trade};
use crate::enums::orderbook_update_type::OrderbookUpdateType;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderbookUpdate {
    pub symbol: u128,
    pub update_type: OrderbookUpdateType,
    pub order: Option<Order>,
    pub trade: Option<Trade>,
    pub cancel_id: Option<u128>,
    pub filled_id: Option<u128>,
}
