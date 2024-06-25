## High-Performance Orderbook in Rust


### Overview

This repository contains a high-performance orderbook implementation written in Rust, capable of processing up to 1 000_000 orders per second (Macbook  Air 2014 with 4go of memory 1600 Mhz DDR3, Intel HD Graphics 5000 1536 Mo).
The orderbook is designed to efficiently handle a large number of buy and sell orders for a given financial instrument, ensuring low latency and high throughput.


### Features
- High-speed processing: The orderbook can handle up to 1,000,000 orders per second.
- Limit order and Market Order available.
- Concurrency : wrap the orderbooks_manager around a RwLock to use it in concurency setup.
- Order Matching: Matches buy and sell orders based on price.
- Order Cancellation : Supports the cancellation of orders before they are matched.
- Order Update: Supports the update of orders before they are matched (amend quantity and price).
- Message Queue: Each state produce a message that you can listen an react to.
- Orderbook summary: Support orderbook summary generation for displaying an UI orderbook (Price levels)

***the repo use ulid to generate IDs, please add it into your project if you intend to use this orderbook implementation***

````rust
use orderbook::OrderbooksManager;
use orderbook::OrderType;
use orderbook::OrderSide;
use orderbook::Order;
use ulid::Ulid;

let mut orderbooks_manager = OrderbooksManager::new();
let symbol: u128 = Ulid::new().into();
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

```
