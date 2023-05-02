use super::{Asks, Bids};
use crate::ops::{update_strategies::ReplaceOrRemove, Update};
use crate::PriceAndQuantity;
#[cfg(feature = "serde")]
use serde::Deserialize;

mod deserialize;

#[cfg(feature = "prost")]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos.rs"));

    use super::LimitOrderBook as NativeLOB;

    impl From<NativeLOB> for LimitOrderBook {
        fn from(og: NativeLOB) -> Self {
            LimitOrderBook {
                update_id: og.update_id as u64,
                bids: Some(Bids {
                    bids: og
                        .bids
                        .iter()
                        .map(|p_n_q| PriceAndQuantity {
                            price: p_n_q.0,
                            quantity: p_n_q.1,
                        })
                        .collect(),
                }),
                asks: Some(Asks {
                    asks: og
                        .asks
                        .iter()
                        .map(|p_n_q| PriceAndQuantity {
                            price: p_n_q.0,
                            quantity: p_n_q.1,
                        })
                        .collect(),
                }),
            }
        }
    }

    impl From<LimitOrderBook> for NativeLOB {
        fn from(book: LimitOrderBook) -> Self {
            let LimitOrderBook {
                update_id,
                bids,
                asks,
            } = book;

            let bids: super::Bids = bids
                .map_or_else(
                    || vec![],
                    |bids| {
                        bids.bids
                            .into_iter()
                            .map(|PriceAndQuantity { price, quantity }| {
                                super::PriceAndQuantity(price, quantity)
                            })
                            .collect()
                    },
                )
                .into();

            let asks: super::Asks = asks
                .map_or_else(
                    || vec![],
                    |asks| {
                        asks.asks
                            .into_iter()
                            .map(|PriceAndQuantity { price, quantity }| {
                                super::PriceAndQuantity(price, quantity)
                            })
                            .collect()
                    },
                )
                .into();

            Self {
                update_id,
                bids,
                asks,
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(PartialEq, Clone, Debug, Default)]
pub struct LimitOrderBook {
    #[serde(alias = "lastUpdateId")]
    pub update_id: u64,
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
    pub timestamp: u64,
    #[serde(alias = "U")]
    pub first_update_id: u64,
    #[serde(alias = "u")]
    pub last_update_id: u64,
    #[serde(alias = "b")]
    pub bids: Bids,
    #[serde(alias = "a")]
    pub asks: Asks,
}
