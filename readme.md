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
- Message Queue: Each state produce a message that you can listen an react to.

