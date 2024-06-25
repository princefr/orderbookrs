use super::orderbook_update::OrderbookUpdate;
use super::trade::Trade;
use crate::enums::order_type::OrderType;
use crate::enums::orderbook_update_type::OrderbookUpdateType;
use crate::enums::side::OrderSide;
use crate::heap::main::ModifiableBinaryHeap;
use crate::structs::order::Order;
use crossbeam_channel::Sender;

#[derive(Debug, Clone)]
pub struct Orderbook {
    pub symbol: u128,
    pub bids: ModifiableBinaryHeap<Order>,
    pub asks: ModifiableBinaryHeap<Order>,
    pub tx: Sender<OrderbookUpdate>,
}

impl Orderbook {
    /// Create a new orderbook
    /// 
    /// #Parameters
    /// 
    /// * 'symbol' - The symbol ID
    /// * 'tx' - The channel Sender [please refer to crossbeam_channel] 
    /// 
    /// #Returns
    /// * 'Orderbook' - The instance of the orderbook
    pub fn new(symbol: u128, tx: Sender<OrderbookUpdate>) -> Orderbook {
        Orderbook {
            symbol,
            bids: ModifiableBinaryHeap::new(),
            asks: ModifiableBinaryHeap::new(),
            tx,
        }
    }

    /// summarize_orderbook_per_price_level returns a tuple of (Vec<(f64, f64, f64)>, f64, Vec<(f64, f64, f64)>) where the first element is a vector of bids, the second element is the mid price and the third element is a vector of asks
    pub fn summarize_orderbook_per_price_level(
        &self,
    ) -> (Vec<(f64, f64, f64)>, f64, Vec<(f64, f64, f64)>) {
        let mut asks = Vec::new();
        let mut bids = Vec::new();
        let mut ask_sum = 0.0;
        let mut bid_sum = 0.0;
        for ask in self.asks.into_vec().iter() {
            ask_sum += ask.quantity;
            asks.push((ask.price.unwrap(), ask.quantity, ask_sum));
        }
        for bid in self.bids.iter_sorted().iter() {
            bid_sum += bid.quantity;
            bids.push((bid.price.unwrap(), bid.quantity, bid_sum));
        }
        bids.reverse();
        (bids, self.get_mid_price(), asks)
    }

    /// get_mid_price returns the mid price of the orderbook
    /// 
    /// #Returns
    /// * f64 - The middle price 
    pub fn get_mid_price(&self) -> f64 {
        let bid = self.bids.peek();
        let ask = self.asks.peek();
        match (bid, ask) {
            (Some(bid), Some(ask)) => (bid.price.unwrap() + ask.price.unwrap()) / 2.0,
            _ => 0.0,
        }
    }

    /// place an order in the orderbook
    pub fn place_order(&mut self, order: Order) {
        match order.side {
            OrderSide::Buy => self.bids.push(order),
            OrderSide::Sell => self.asks.push(order),
        }
        self.tx
            .send(OrderbookUpdate {
                symbol: self.symbol,
                update_type: OrderbookUpdateType::Place,
                order: Some(order),
                trade: None,
                cancel_id: None,
                filled_id: None,
            })
            .unwrap();
        self.match_orders();
    }

    /// match_orders matches the orders in the orderbook
    pub fn amend_order_price(&mut self, order_id: u128, new_price: f64, order_side: OrderSide) {
        let mut order: Option<Order> = None;
        match order_side {
            OrderSide::Buy => {
                self.bids.modify(|o| {
                    if o.id == order_id {
                        o.price = Some(new_price);
                        order = Some(*o);
                    }
                });
            }
            OrderSide::Sell => {
                self.asks.modify(|o| {
                    if o.id == order_id {
                        o.price = Some(new_price);
                        order = Some(*o);
                    }
                });
            }
        };
        self.tx
            .send(OrderbookUpdate {
                symbol: self.symbol,
                update_type: OrderbookUpdateType::Update,
                order,
                trade: None,
                cancel_id: None,
                filled_id: None,
            })
            .unwrap();
        self.match_orders();
    }

    ///amend_order_quantity amends the quantity of an order in the orderbook
    pub fn amend_order_quantity(
        &mut self,
        order_id: u128,
        new_quantity: f64,
        order_side: OrderSide,
    ) {
        let mut order: Option<Order> = None;
        match order_side {
            OrderSide::Buy => {
                self.bids.modify(|o| {
                    if o.id == order_id {
                        o.quantity = new_quantity;
                        order = Some(*o);
                    }
                });
            }
            OrderSide::Sell => {
                self.asks.modify(|o| {
                    if o.id == order_id {
                        o.quantity = new_quantity;
                        order = Some(*o);
                    }
                });
            }
        }
        self.tx
            .send(OrderbookUpdate {
                symbol: self.symbol,
                update_type: OrderbookUpdateType::Update,
                order,
                trade: None,
                cancel_id: None,
                filled_id: None,
            })
            .unwrap();
        self.match_orders();
    }

    /// update_order updates the quantity of an order in the orderbook
    pub fn update_order(&mut self, order_id: u128, new_quantity: f64, order_side: OrderSide) {
        let mut order: Option<Order> = None;
        match order_side {
            OrderSide::Buy => {
                self.bids.modify(|o| {
                    if o.id == order_id {
                        o.quantity = new_quantity;
                        order = Some(*o);
                    }
                });
            }
            OrderSide::Sell => {
                self.asks.modify(|o| {
                    if o.id == order_id {
                        o.quantity = new_quantity;
                        order = Some(*o);
                    }
                });
            }
        }

        self.tx
            .send(OrderbookUpdate {
                symbol: self.symbol,
                update_type: OrderbookUpdateType::Update,
                order,
                trade: None,
                cancel_id: None,
                filled_id: None,
            })
            .unwrap();
    }

    /// match orders in the orderbook
    pub fn match_orders(&mut self) {
        while let Some(ask) = self.asks.peek() {
            if let Some(bid) = self.bids.peek() {
                if bid.price >= ask.price {
                    if ask.quantity > bid.quantity {
                        self.order_filled(bid.id, bid.side);
                        self.update_order(ask.id, ask.quantity - bid.quantity, ask.side);
                        let trade = Trade {
                            id: None,
                            symbol: self.symbol,
                            price: ask.price.unwrap(),
                            quantity: bid.quantity,
                            buy_order_id: bid.id,
                            sell_order_id: ask.id,
                            buy_user_id: bid.user_id,
                            sell_user_id: ask.user_id,
                            status: Default::default(),
                            created_at: None,
                            updated_at: None,
                        };
                        self.tx
                            .send(OrderbookUpdate {
                                symbol: self.symbol,
                                update_type: OrderbookUpdateType::NewTrades,
                                order: None,
                                trade: Some(trade),
                                cancel_id: None,
                                filled_id: None,
                            })
                            .unwrap();
                    } else if ask.quantity < bid.quantity {
                        self.order_filled(ask.id, ask.side);
                        self.update_order(bid.id, bid.quantity - ask.quantity, bid.side);
                        let trade = Trade {
                            id: None,
                            symbol: self.symbol,
                            price: ask.price.unwrap(),
                            quantity: ask.quantity,
                            buy_order_id: bid.id,
                            sell_order_id: ask.id,
                            buy_user_id: bid.user_id,
                            sell_user_id: ask.user_id,
                            status: Default::default(),
                            created_at: None,
                            updated_at: None,
                        };
                        self.tx
                            .send(OrderbookUpdate {
                                symbol: self.symbol,
                                update_type: OrderbookUpdateType::NewTrades,
                                order: None,
                                trade: Some(trade),
                                cancel_id: None,
                                filled_id: None,
                            })
                            .unwrap();
                    } else {
                        self.order_filled(ask.id, ask.side);
                        self.order_filled(bid.id, bid.side);
                        let trade = Trade {
                            id: None,
                            symbol: self.symbol,
                            price: ask.price.unwrap(),
                            quantity: ask.quantity,
                            buy_order_id: bid.id,
                            sell_order_id: ask.id,
                            buy_user_id: bid.user_id,
                            sell_user_id: ask.user_id,
                            status: Default::default(),
                            created_at: None,
                            updated_at: None,
                        };
                        self.tx
                            .send(OrderbookUpdate {
                                symbol: self.symbol,
                                update_type: OrderbookUpdateType::NewTrades,
                                order: None,
                                trade: Some(trade),
                                cancel_id: None,
                                filled_id: None,
                            })
                            .unwrap();
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    /// cancel_order cancels an order in the orderbook
    pub fn cancel_order(&mut self, order_id: u128, order_side: OrderSide) {
        match order_side {
            OrderSide::Buy => {
                self.bids.retain(|o| o.id != order_id);
            }
            OrderSide::Sell => {
                self.asks.retain(|o| o.id != order_id);
            }
        }
        self.tx
            .send(OrderbookUpdate {
                symbol: self.symbol,
                update_type: OrderbookUpdateType::Cancel,
                order: None,
                trade: None,
                cancel_id: Some(order_id),
                filled_id: None,
            })
            .unwrap();
    }

    /// order_filled marks an order as filled in the orderbook
    pub fn order_filled(&mut self, order_id: u128, order_side: OrderSide) {
        match order_side {
            OrderSide::Buy => {
                self.bids.retain(|o| o.id != order_id);
            }
            OrderSide::Sell => {
                self.asks.retain(|o| o.id != order_id);
            }
        }
        self.tx
            .send(OrderbookUpdate {
                symbol: self.symbol,
                update_type: OrderbookUpdateType::Filled,
                order: None,
                trade: None,
                cancel_id: None,
                filled_id: Some(order_id),
            })
            .unwrap();
    }

    /// add_order adds an order to the orderbook without matching it
    pub fn add_order(&mut self, order: Order) {
        self.tx
            .send(OrderbookUpdate {
                symbol: self.symbol,
                update_type: OrderbookUpdateType::New,
                order: Some(order),
                trade: None,
                cancel_id: None,
                filled_id: None,
            })
            .unwrap();
        match order.order_type {
            OrderType::Limit => self.place_order(order),
            OrderType::Market => {
                let mut quantity = order.quantity;
                if order.side == OrderSide::Buy {
                    while let Some(ask) = self.asks.peek() {
                        if ask.quantity <= quantity {
                            self.order_filled(ask.id, ask.side);
                            quantity -= ask.quantity;
                            let trade = Trade {
                                id: None,
                                symbol: self.symbol,
                                price: ask.price.unwrap(),
                                quantity: ask.quantity,
                                buy_order_id: order.id,
                                sell_order_id: ask.id,
                                buy_user_id: order.user_id,
                                sell_user_id: ask.user_id,
                                status: Default::default(),
                                created_at: None,
                                updated_at: None,
                            };
                            self.tx
                                .send(OrderbookUpdate {
                                    symbol: self.symbol,
                                    update_type: OrderbookUpdateType::NewTrades,
                                    order: None,
                                    trade: Some(trade),
                                    filled_id: None,
                                    cancel_id: None,
                                })
                                .unwrap();
                        } else {
                            self.update_order(ask.id, ask.quantity - quantity, ask.side);
                            let trade = Trade {
                                id: None,
                                symbol: self.symbol,
                                price: ask.price.unwrap(),
                                quantity,
                                buy_order_id: order.id,
                                sell_order_id: ask.id,
                                buy_user_id: order.user_id,
                                sell_user_id: ask.user_id,
                                status: Default::default(),
                                created_at: None,
                                updated_at: None,
                            };
                            self.tx
                                .send(OrderbookUpdate {
                                    symbol: self.symbol,
                                    update_type: OrderbookUpdateType::NewTrades,
                                    order: None,
                                    trade: Some(trade),
                                    filled_id: None,
                                    cancel_id: None,
                                })
                                .unwrap();
                            break;
                        }
                    }
                } else {
                    while let Some(bid) = self.bids.peek() {
                        if bid.quantity <= quantity {
                            quantity -= bid.quantity;
                            self.order_filled(bid.id, bid.side);
                            let trade = Trade {
                                id: None,
                                symbol: self.symbol,
                                price: bid.price.unwrap(),
                                quantity: bid.quantity,
                                buy_order_id: bid.id,
                                sell_order_id: order.id,
                                buy_user_id: bid.user_id,
                                sell_user_id: order.user_id,
                                status: Default::default(),
                                created_at: None,
                                updated_at: None,
                            };
                            self.tx
                                .send(OrderbookUpdate {
                                    symbol: self.symbol,
                                    update_type: OrderbookUpdateType::NewTrades,
                                    order: None,
                                    trade: Some(trade),
                                    filled_id: None,
                                    cancel_id: None,
                                })
                                .unwrap();
                        } else {
                            self.update_order(bid.id, bid.quantity - quantity, bid.side);
                            let trade = Trade {
                                id: None,
                                symbol: self.symbol,
                                price: bid.price.unwrap(),
                                quantity,
                                buy_order_id: bid.id,
                                sell_order_id: order.id,
                                buy_user_id: bid.user_id,
                                sell_user_id: order.user_id,
                                status: Default::default(),
                                created_at: None,
                                updated_at: None,
                            };
                            self.tx
                                .send(OrderbookUpdate {
                                    symbol: self.symbol,
                                    update_type: OrderbookUpdateType::NewTrades,
                                    order: None,
                                    trade: Some(trade),
                                    filled_id: None,
                                    cancel_id: None,
                                })
                                .unwrap();
                            break;
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    
    use std::time::Instant;
    

    use super::*;
    use crate::enums::order_type::OrderType;
    use crate::enums::side::OrderSide;
    use crate::structs::order::Order;
    use crossbeam_channel::unbounded;
    use ulid::Ulid;

    #[test]
    fn test_orderbook_new() {
        let (tx, _) = unbounded::<OrderbookUpdate>();
        let orderbook = Orderbook::new(Ulid::new().into(), tx);
        assert_eq!(orderbook.bids.len(), 0);
        assert_eq!(orderbook.asks.len(), 0);
    }

    #[test]
    fn test_orderbook_add_order() {
        let (tx, r) = unbounded::<OrderbookUpdate>();
        let mut orderbook = Orderbook::new(Ulid::new().into(), tx);
        let order = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        // Spawn a new thread to keep the receiver alive
        std::thread::spawn(move || {
            println!("{:?}", r.recv().unwrap());
        });
        orderbook.add_order(order.clone());
        assert_eq!(orderbook.bids.len(), 1);
        assert_eq!(orderbook.asks.len(), 0);
    }

    #[test]
    fn test_orderbook_update_order() {
        let (tx, r) = unbounded::<OrderbookUpdate>();
        let mut orderbook = Orderbook::new(Ulid::new().into(), tx);
        let order = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        std::thread::spawn(move || loop {
            println!("{:?}", r.recv().unwrap());
        });
        orderbook.add_order(order.clone());
        orderbook.update_order(order.id, 2.0, OrderSide::Buy);
        assert_eq!(orderbook.bids.len(), 1);
        assert_eq!(orderbook.asks.len(), 0);
        let new_order = orderbook.bids.peek().unwrap();
        assert_eq!(new_order.quantity, 2.0);
    }

    #[test]
    fn test_case_1() {
        let (tx, r) = unbounded::<OrderbookUpdate>();
        let mut orderbook = Orderbook::new(Ulid::new().into(), tx);
        let order1 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            1.0,
            Some(1.0),
            OrderType::Limit,
        );
        std::thread::spawn(move || loop {
            println!("{:?}", r.recv().unwrap());
        });
        orderbook.add_order(order1.clone());
        orderbook.add_order(order2.clone());
        assert_eq!(orderbook.bids.len(), 0);
        assert_eq!(orderbook.asks.len(), 0);
    }

    #[test]
    fn test_case_2() {
        let (tx, r) = unbounded::<OrderbookUpdate>();
        let mut orderbook = Orderbook::new(Ulid::new().into(), tx);
        let order1 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            100.10,
            Some(100.10),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            500.0,
            Some(100.05),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            1000.0,
            Some(100.0),
            OrderType::Limit,
        );
        std::thread::spawn(move || loop {
            println!("{:?}", r.recv().unwrap());
        });
        orderbook.add_order(order1.clone());
        orderbook.add_order(order2.clone());
        orderbook.add_order(order3.clone());
        let order1 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            100.0,
            Some(99.95),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            50.0,
            Some(99.90),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            50.0,
            Some(99.85),
            OrderType::Limit,
        );

        orderbook.add_order(order1.clone());
        orderbook.add_order(order2.clone());
        orderbook.add_order(order3.clone());
        assert_eq!(orderbook.bids.len(), 3);
        assert_eq!(orderbook.asks.len(), 3);

        let order = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            100.0,
            Some(100.0),
            OrderType::Market,
        );

        orderbook.add_order(order.clone());
        assert_eq!(orderbook.bids.len(), 3);
        assert_eq!(orderbook.asks.len(), 3);
        let order = orderbook.asks.peek().unwrap();
        assert_eq!(order.quantity, 900.0);
        assert_eq!(order.price, Some(100.0));
        assert_eq!(orderbook.get_mid_price(), 99.975);
    }

    #[test]
    fn test_case_3() {
        let (tx, r) = unbounded::<OrderbookUpdate>();
        let mut orderbook = Orderbook::new(Ulid::new().into(), tx);
        let order1 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            100.10,
            Some(100.10),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            500.0,
            Some(100.05),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            900.0,
            Some(100.0),
            OrderType::Limit,
        );
        std::thread::spawn(move || loop {
            println!("{:?}", r.recv().unwrap());
        });
        orderbook.add_order(order1.clone());
        orderbook.add_order(order2.clone());
        orderbook.add_order(order3.clone());
        let order1 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            100.0,
            Some(99.95),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            50.0,
            Some(99.90),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            50.0,
            Some(99.85),
            OrderType::Limit,
        );

        orderbook.add_order(order1.clone());
        orderbook.add_order(order2.clone());
        orderbook.add_order(order3.clone());
        assert_eq!(orderbook.bids.len(), 3);
        assert_eq!(orderbook.asks.len(), 3);

        let order = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            100.0,
            Some(100.02),
            OrderType::Limit,
        );

        orderbook.add_order(order.clone());
        assert_eq!(orderbook.bids.len(), 3);
        assert_eq!(orderbook.asks.len(), 3);
        let order = orderbook.asks.peek().unwrap();
        assert_eq!(order.quantity, 800.0);
        assert_eq!(order.price, Some(100.0));
    }

    #[test]
    fn test_case_4() {
        let (tx, r) = unbounded::<OrderbookUpdate>();
        let mut orderbook = Orderbook::new(Ulid::new().into(), tx);
        let order1 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            100.10,
            Some(100.10),
            OrderType::Limit,
        );
        let order2 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            500.0,
            Some(100.05),
            OrderType::Limit,
        );
        let order3 = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Sell,
            900.0,
            Some(100.0),
            OrderType::Limit,
        );
        std::thread::spawn(move || loop {
            println!("{:?}", r.recv().unwrap());
        });
        orderbook.add_order(order1.clone());
        orderbook.add_order(order2.clone());
        orderbook.add_order(order3.clone());

        let order = Order::new(
            Ulid::new().into(),
            Ulid::new().into(),
            OrderSide::Buy,
            2000.0,
            Some(100.0),
            OrderType::Market,
        );
        orderbook.add_order(order.clone());

        assert_eq!(orderbook.bids.len(), 0);
        assert_eq!(orderbook.asks.len(), 0);
    }


    #[test]
    fn test_benchmark() {
        let (tx, r) = unbounded::<OrderbookUpdate>();
        let mut orderbook = Orderbook::new(Ulid::new().into(), tx);
        std::thread::spawn(move || loop {
            if let Ok(_update) = r.recv() {}
        });

        let mut orders = Vec::new();
        let start = Instant::now();
        for i in 0..1000000 {
            let order = Order::new(
                Ulid::new().into(),
                Ulid::new().into(),
                OrderSide::Sell,
                100.10 + i as f64,
                Some(100.10),
                OrderType::Limit,
            );
            orders.push(order);
        }
        let duration = start.elapsed();
        println!(
            "Time elapsed in looping and creating 1,000,000 orders is: {:?}",
            duration
        );

        let start = Instant::now();
        for order in orders {
            orderbook.add_order(order);
        }
        let duration = start.elapsed();
        println!("Time elapsed in adding 1,000,000 orders is: {:?}", duration);

        assert_eq!(orderbook.asks.len(), 1000000);
    }
}
