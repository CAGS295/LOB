use super::{Asks, Bids};

#[derive(PartialEq, Debug, Clone)]
pub struct LimitOrderBook {
    bids: Bids,
    asks: Asks,
}
