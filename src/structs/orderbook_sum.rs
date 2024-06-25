
#[derive(Debug, PartialEq)]
pub struct BidAskSummarize {
    pub price: f64,
    pub qty: f64,
    pub qty_sum: f64,
    pub qty_percent: f64,
}

impl BidAskSummarize {
    pub fn new(price: f64, qty: f64, qty_sum: f64, qty_percent: f64) -> BidAskSummarize {
        BidAskSummarize {
            price,
            qty,
            qty_sum,
            qty_percent,
        }
    }
}

#[derive(Debug)]
pub struct OrderBookSummarized {
    pub bids: Vec<BidAskSummarize>,
    pub mid_price: f64,
    pub asks: Vec<BidAskSummarize>,
}

impl OrderBookSummarized {
    pub fn new(
        bids: Vec<(f64, f64, f64)>,
        mid_price: f64,
        asks: Vec<(f64, f64, f64)>,
    ) -> OrderBookSummarized {
        let bids_volume: f64 = bids.iter().map(|b| b.1).sum();
        let bids = bids
            .iter()
            .map(|b| BidAskSummarize::new(b.0, b.1, b.2, b.1 / bids_volume * 100.0))
            .collect();

        let asks_volume: f64 = asks.iter().map(|a| a.1).sum();
        let asks = asks
            .iter()
            .map(|a| BidAskSummarize::new(a.0, a.1, a.2, a.1 / asks_volume * 100.0))
            .collect();
        OrderBookSummarized {
            bids,
            mid_price,
            asks,
        }
    }
}
