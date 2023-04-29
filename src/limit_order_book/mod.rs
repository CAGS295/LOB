use crate::ops::{update_strategies::ReplaceOrRemove, Update};
use crate::PriceAndQuantity;

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

    pub fn add_bid(&mut self, bid: PriceAndQuantity<f64, f64>) {
        Update::<ReplaceOrRemove>::insert(&mut self.bids, bid)
    }

    pub fn add_ask(&mut self, ask: PriceAndQuantity<f64, f64>) {
        Update::<ReplaceOrRemove>::insert(&mut self.asks, ask)
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
    pub bids: Bids,
    #[serde(alias = "a")]
    pub asks: Asks,
}
