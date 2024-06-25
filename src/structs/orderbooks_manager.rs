use super::orderbook::Orderbook;
use super::orderbook_update::OrderbookUpdate;
use super::trade::Trade;
use crate::structs::order::Order;
use crate::structs::orderbook_sum::{BidAskSummarize, OrderBookSummarized};
use crate::{OrderSide, OrderbookUpdateType};
use async_stream::stream;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures_util::Stream;
use std::collections::HashMap;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct OrderbooksManager {
    pub orderbooks: HashMap<u128, Orderbook>,
    pub tx: Sender<OrderbookUpdate>,
    pub rx: Receiver<OrderbookUpdate>,
}

impl OrderbooksManager {
    /// Create a new OrderbooksManager that will manage all orderbooks
    ///
    /// #Returns
    /// * OrderbooksManager - An instance of the orderbook manager
    pub fn new() -> OrderbooksManager {
        let (tx, rx) = unbounded::<OrderbookUpdate>();
        OrderbooksManager {
            orderbooks: HashMap::new(),
            tx,
            rx,
        }
    }

    /// Create a new orderbook with a symbol
    ///
    /// Parameters
    /// * 'symbol' : The symbol ID the new orderbook will be in
    pub fn new_orderbook<'a>(&mut self, symbol: u128) {
        let exist = self.get_orderbook(symbol).is_ok();
        assert!(exist == false, "the orderbook already exist");
        // Todo!("assert or something else?")
        let orderbook = Orderbook::new(symbol, self.tx.clone());
        self.orderbooks.insert(symbol, orderbook);
    }

    /// Add an order to the orderbook
    ///
    /// Parameters
    /// * 'symbol' : The symbol ID
    pub fn add_order<'a>(&mut self, order: Order) -> Result<(), Error> {
        if let Some(orderbook) = self.orderbooks.get_mut(&order.symbol) {
            orderbook.add_order(order);
            return Ok(());
        }
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Orderbook not found",
        ))
    }

    /// Amend an order price in the orderbook
    ///
    /// Parameters
    /// * 'symbol' : The symbol ID
    /// * 'order_id': The order ID to ammend
    /// * 'price': The new price of the order
    /// * 'side': The order side
    pub fn amend_order_price<'a>(
        &mut self,
        symbol: u128,
        order_id: u128,
        price: f64,
        side: OrderSide,
    ) -> Result<(), Error> {
        if let Some(orderbook) = self.orderbooks.get_mut(&symbol) {
            orderbook.amend_order_price(order_id, price, side);
            return Ok(());
        }
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Orderbook not found",
        ))
    }

    /// Amend an order quanitty in the orderbook
    ///
    /// #Parameters
    /// * 'symbol' : The symbol ID
    /// * 'order_id': The order ID to ammend
    /// * 'quantity': The new quantity of the order
    /// * 'side': The order side
    pub fn amend_order_quantity<'a>(
        &mut self,
        symbol: u128,
        order_id: u128,
        quantity: f64,
        side: OrderSide,
    ) -> Result<(), Error> {
        if let Some(orderbook) = self.orderbooks.get_mut(&symbol) {
            orderbook.amend_order_quantity(order_id, quantity, side);
            return Ok(());
        }
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Orderbook not found",
        ))
    }

    /// Cancel the order by order_id
    ///
    /// #Parameters
    /// * 'order_id' - The order ID
    /// * 'symbol' - The symbol ID
    /// * 'side'- The order side
    pub fn cancel_order<'a>(
        &mut self,
        order_id: u128,
        symbol: u128,
        side: OrderSide,
    ) -> Result<(), Error> {
        if let Some(orderbook) = self.orderbooks.get_mut(&symbol) {
            orderbook.cancel_order(order_id, side);
            return Ok(());
        }
        Err(Error::new(std::io::ErrorKind::NotFound, "Order not found"))
    }

    /// Get an orderbook summary by symbol
    ///
    /// Parameters
    /// * 'symbol' - The symbol ID
    pub fn get_orderbook(&self, symbol: u128) -> Result<OrderBookSummarized, Error> {
        if let Some(orderbook) = self.orderbooks.get(&symbol) {
            let summary = orderbook.summarize_orderbook_per_price_level();
            let bids_volume: f64 = summary.0.iter().map(|b| b.1).sum();
            let asks_volume: f64 = summary.2.iter().map(|a| a.1).sum();
            let bids = summary
                .0
                .iter()
                .map(|b| BidAskSummarize::new(b.0, b.1, b.2, b.1 / bids_volume * 100.0))
                .collect();
            let asks = summary
                .2
                .iter()
                .map(|a| BidAskSummarize::new(a.0, a.1, a.2, a.1 / asks_volume * 100.0))
                .collect();
            let summary_back = OrderBookSummarized {
                bids,
                asks,
                mid_price: summary.1,
            };
            return Ok(summary_back);
        }
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Orderbook not found",
        ))
    }

    /// Listen to new orders
    pub fn listen_new_orders<'a>(&'a self) -> impl Stream<Item = Order> {
        let rx = self.rx.clone();
        stream! {

                    while let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::New => {
                                if let Some(order) = orderbook_update.order {
                                    yield order;
                                }
                            }
                            _ => {}
                        }
                    }

        }
    }

    /// Listen to placed orders
    pub fn listen_placed_orders<'a>(&'a self) -> impl Stream<Item = Order> {
        let rx = self.rx.clone();
        stream! {

                    while let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::Place => {
                                if let Some(order) = orderbook_update.order {
                                    yield order;
                                }
                            }
                            _ => {}
                        }
                    }

        }
    }

    /// Listen to new trades
    pub fn listen_new_trades<'a>(&self) -> impl Stream<Item = Trade> {
        let rx = self.rx.clone();
        stream! {

                    while let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::NewTrades => {
                                if let Some(trade) = orderbook_update.trade {
                                    yield trade;
                                }
                            }
                            _ => {}

                        }
                    }
        }
    }

    /// listen to orderbook summary by symbol
    pub fn listen_orderbook_summary_by_symbol<'a>(
        &'a self,
        symbol: u128,
    ) -> impl Stream<Item = OrderBookSummarized> + 'a {
        let rx = self.rx.clone();
        stream! {
                    while let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::Place => {

                                if orderbook_update.symbol == symbol {
                                    if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                        yield summary_back;
                                    }
                                }
                            }
                            OrderbookUpdateType::Cancel => {
                                if orderbook_update.symbol == symbol {
                                    if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                        yield summary_back;
                                    }
                                }

                            }
                            OrderbookUpdateType::Update=> {
                                if orderbook_update.symbol == symbol {
                                    if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                        yield summary_back;
                                    }
                                }

                            },
                            OrderbookUpdateType::Filled=> {
                                if orderbook_update.symbol == symbol {
                                    if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                        yield summary_back;
                                    }
                                }
                            },

                            _ => {}

                        }
                    }

        }
    }

    /*
    * Listen to orderbook summary
        @return impl Stream<Item = OrderBookSummarized>
    */
    pub fn listen_orderbook_summary<'a>(&'a self) -> impl Stream<Item = OrderBookSummarized> + 'a {
        let rx = self.rx.clone();
        stream! {

                    while let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::Place => {
                                if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                    yield summary_back;
                                }                          }
                            OrderbookUpdateType::Cancel => {
                                if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                    yield summary_back;
                                }

                            }
                            OrderbookUpdateType::Update=> {
                                if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                    yield summary_back;
                                }

                            },
                            OrderbookUpdateType::Filled=> {
                                if let Ok(summary_back) = self.get_orderbook(orderbook_update.symbol) {
                                    yield summary_back;
                                }
                            },

                            _ => {}

                        }
                    }

        }
    }

    /// Listen to orderbook updates
    pub fn listen_orderbook_updates<'a>(&self) -> impl Stream<Item = Order> {
        let rx = self.rx.clone();
        stream! {
                loop {
                    if let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::Update=> {
                                if let Some(order) = orderbook_update.order {
                                    yield order;
                                }
                            }
                            _ => {}

                        }
                    }
            }
        }
    }

    /// Listen to orderbook cancels
    pub fn listen_orderbook_cancels<'a>(&self) -> impl Stream<Item = u128> {
        let rx = self.rx.clone();
        stream! {

                    while let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::Cancel => {
                                if let Some(id) = orderbook_update.cancel_id {
                                    yield id;
                                }
                            }
                            _ => {}
                        }
                    }

        }
    }

    /// Listen to orderbook fills
    pub fn listen_orderbook_fills<'a>(&self) -> impl Stream<Item = u128> {
        let rx = self.rx.clone();
        stream! {

                    while let Ok(orderbook_update) = rx.recv() {
                        match orderbook_update.update_type {
                            OrderbookUpdateType::Filled => {
                                if let Some(id) = orderbook_update.filled_id {
                                    yield id;
                                }
                            }
                            _ => {}

                        }
                    }

        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::enums::order_type::OrderType;
    use crate::enums::side::OrderSide;
    use crate::structs::order::Order;
    use futures_util::StreamExt;
    use ulid::Ulid;

    #[tokio::test]
    async fn test_listen_to_new_orders() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.add_order(order2.clone());
        let _ = orderbooks_manager.add_order(order3.clone());

        let mut new_orders_stream = orderbooks_manager.listen_new_orders().boxed();

        let first_order = new_orders_stream.next().await.unwrap();
        assert_eq!(first_order, order1);

        let second_order = new_orders_stream.next().await.unwrap();
        assert_eq!(second_order, order2);

        let third_order = new_orders_stream.next().await.unwrap();
        assert_eq!(third_order, order3);
    }

    #[tokio::test]
    async fn test_listen_to_placed_orders() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.add_order(order2.clone());
        let _ = orderbooks_manager.add_order(order3.clone());

        let mut new_orders_stream = orderbooks_manager.listen_placed_orders().boxed();

        let first_order = new_orders_stream.next().await.unwrap();
        assert_eq!(first_order, order1);

        let second_order = new_orders_stream.next().await.unwrap();
        assert_eq!(second_order, order2);

        let third_order = new_orders_stream.next().await.unwrap();
        assert_eq!(third_order, order3);
    }

    #[tokio::test]
    async fn test_listen_to_orderbook_summary() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(2.0),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(3.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.add_order(order2.clone());
        let _ = orderbooks_manager.add_order(order3.clone());

        let mut new_orders_stream = orderbooks_manager.listen_orderbook_summary().boxed();

        let summary = new_orders_stream.next().await.unwrap();
        assert_eq!(
            summary.bids[0],
            BidAskSummarize {
                price: 3.0,
                qty: 1.0,
                qty_sum: 3.0,
                qty_percent: 33.33333333333333
            }
        );
        assert_eq!(
            summary.bids[1],
            BidAskSummarize {
                price: 2.0,
                qty: 1.0,
                qty_sum: 2.0,
                qty_percent: 33.33333333333333
            }
        );
        assert_eq!(
            summary.bids[2],
            BidAskSummarize {
                price: 1.0,
                qty: 1.0,
                qty_sum: 1.0,
                qty_percent: 33.33333333333333
            }
        );
    }

    #[tokio::test]
    async fn test_listen_to_orderbook_summary_sell() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Sell,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Sell,
            1.0,
            Some(2.0),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Sell,
            1.0,
            Some(3.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.add_order(order2.clone());
        let _ = orderbooks_manager.add_order(order3.clone());

        let mut new_orders_stream = orderbooks_manager.listen_orderbook_summary().boxed();

        let summary = new_orders_stream.next().await.unwrap();
        assert_eq!(
            summary.asks[0],
            BidAskSummarize {
                price: 1.0,
                qty: 1.0,
                qty_sum: 1.0,
                qty_percent: 33.33333333333333
            }
        );
        assert_eq!(
            summary.asks[1],
            BidAskSummarize {
                price: 2.0,
                qty: 1.0,
                qty_sum: 2.0,
                qty_percent: 33.33333333333333
            }
        );
        assert_eq!(
            summary.asks[2],
            BidAskSummarize {
                price: 3.0,
                qty: 1.0,
                qty_sum: 3.0,
                qty_percent: 33.33333333333333
            }
        );
    }

    #[tokio::test]
    async fn test_listen_to_orderbook_updates_amend_price() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.amend_order_price(symbol, order1.id, 50.0, order1.side);

        let mut new_orders_stream = orderbooks_manager.listen_orderbook_updates().boxed();

        let first_order = new_orders_stream.next().await.unwrap();
        assert_eq!(first_order.price, Some(50.0));
    }

    #[tokio::test]
    async fn test_listen_to_orderbook_updates_amend_quantity() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.amend_order_quantity(symbol, order1.id, 10.0, order1.side);

        let mut new_orders_stream = orderbooks_manager.listen_orderbook_updates().boxed();

        let first_order = new_orders_stream.next().await.unwrap();
        assert_eq!(first_order.quantity, 10.0);
    }

    #[tokio::test]
    async fn test_listen_to_new_trade() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Sell,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.add_order(order2.clone());

        let mut new_orders_stream = orderbooks_manager.listen_new_trades().boxed();

        let trade = new_orders_stream.next().await.unwrap();
        assert_eq!(trade.symbol, order1.symbol);
        assert_eq!(Some(trade.price), order1.price);
        assert_eq!(trade.quantity, order1.quantity);
        assert_eq!(trade.buy_order_id, order1.id);
        assert_eq!(trade.sell_order_id, order2.id);
    }

    #[tokio::test]
    async fn test_listen_to_filled_orders() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Sell,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.add_order(order2.clone());

        let mut new_orders_stream = orderbooks_manager.listen_orderbook_fills().boxed();

        let order = new_orders_stream.next().await.unwrap();
        assert_eq!(order, order2.id);
        let order = new_orders_stream.next().await.unwrap();
        assert_eq!(order, order1.id);
    }

    #[tokio::test]
    async fn test_listen_to_cancelled_orders() {
        let mut orderbooks_manager = OrderbooksManager::new();

        let symbol = Ulid::new().into();
        orderbooks_manager.new_orderbook(symbol);
        let order1 = Order::new(
            Ulid::new().into(),
            symbol,
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );

        let _ = orderbooks_manager.add_order(order1.clone());
        let _ = orderbooks_manager.cancel_order(order1.id, symbol, order1.side);

        let mut new_orders_stream = orderbooks_manager.listen_orderbook_cancels().boxed();

        let first_order = new_orders_stream.next().await.unwrap();
        assert_eq!(first_order, order1.id);
    }
}
