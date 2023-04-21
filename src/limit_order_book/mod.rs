use super::{Asks, Bids};

#[derive(PartialEq, Debug, Clone)]
pub struct LimitOrderBook {
    update_id: usize,
    bids: Bids,
    asks: Asks,
}

impl LimitOrderBook {
    pub fn new() -> LimitOrderBook {
        LimitOrderBook {
            update_id: 0,
            bids: Bids::new(),
            asks: Asks::new(),
        }
    }
}
