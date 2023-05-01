use crate::ops::{update_strategies::ReplaceOrRemove, Update};
use crate::PriceAndQuantity;

use super::{Asks, Bids};
#[cfg(feature = "serde")]
use serde::Deserialize;

mod deserialize;

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct LimitOrderBook {
    #[serde(alias = "lastUpdateId")]
    pub update_id: usize,
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

    // Careful, This is a cheap extend and wont respect Ordering.
    // Use it only if you can guarantee that the concatenation yields an ordered Self.
    // e.g. You concatenate partitions.
    pub fn extend(&mut self, other: &Self) {
        let Self {
            update_id,
            bids,
            asks,
        } = other;
        self.bids.extend(bids.iter().copied());
        self.asks.extend(asks.iter().copied());
        self.update_id = *update_id;
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct DepthUpdate {
    #[serde(alias = "E")]
    pub timestamp: usize,
    #[serde(alias = "U")]
    pub first_update_id: usize,
    #[serde(alias = "u")]
    pub last_update_id: usize,
    #[serde(alias = "b")]
    pub bids: Bids,
    #[serde(alias = "a")]
    pub asks: Asks,
}
