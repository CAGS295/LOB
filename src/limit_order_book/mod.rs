use super::{Asks, Bids};
#[cfg(feature = "serde")]
use serde::Deserialize;

mod deserialize;

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct LimitOrderBook {
    #[serde(alias = "lastUpdateId")]
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
