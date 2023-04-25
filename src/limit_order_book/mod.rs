use super::{Asks, Bids};
#[cfg(feature = "serde")]
use serde::Deserialize;

mod deserialize;

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct LimitOrderBook {
    #[serde(alias = "lastUpdateId")]
    update_id: u64,
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

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct DepthUpdate {
    #[serde(alias = "E")]
    timestamp: usize,
    #[serde(alias = "U")]
    first_update_id: usize,
    #[serde(alias = "u")]
    last_update_id: usize,
    #[serde(alias = "b")]
    bids: Bids,
    #[serde(alias = "a")]
    asks: Asks,
}
