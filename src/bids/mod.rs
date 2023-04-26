#[cfg(feature = "serde")]
mod deserializer;

use super::{ops::Update, PriceAndQuantity};
use crate::ops::{update_strategies::Strategy, BinarySearchPredicate, Updatable};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::ops::{Add, Deref, DerefMut};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(PartialEq, Clone, Debug, Default)]
pub struct Bids<P = f64, Q = f64>(Vec<PriceAndQuantity<P, Q>>);

impl<P, Q> From<Vec<PriceAndQuantity<P, Q>>> for Bids<P, Q> {
    fn from(value: Vec<PriceAndQuantity<P, Q>>) -> Self {
        Self(value)
    }
}

impl<P, Q> Deref for Bids<P, Q> {
    type Target = Vec<PriceAndQuantity<P, Q>>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<P, Q> DerefMut for Bids<P, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl<P, Q> Bids<P, Q> {
    pub fn new() -> Self {
        Vec::new().into()
    }
}

impl<P, Q> Bids<P, Q>
where
    P: Clone + PartialOrd,
    Q: Clone + Add<Output = Q> + Default + PartialEq,
{
    pub fn add_bid<S>(&mut self, bid: PriceAndQuantity<P, Q>)
    where
        S: Strategy,
        Self: Update<S, Tuple<P, Q> = PriceAndQuantity<P, Q>>,
    {
        Update::insert::<P, Q>(self, bid)
    }
}

impl<P, Q> BinarySearchPredicate for Bids<P, Q> {
    fn partition_predicate<Price: PartialOrd>(lhs: &Price, rhs: &Price) -> bool {
        rhs > lhs
    }
}

impl<P, Q> Updatable for Bids<P, Q> {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ops::update_strategies::{AggregateOrCreate, ReplaceOrRemove};

    #[test]
    fn greater_bids_are_pushed_back_asc() {
        let mut bids = Bids::new();
        bids.add_bid::<AggregateOrCreate>(PriceAndQuantity(0., 1));
        bids.add_bid::<AggregateOrCreate>(PriceAndQuantity(1., 1));
        assert_eq!(bids.0, [PriceAndQuantity(0., 1), PriceAndQuantity(1., 1)]);
    }

    #[test]
    fn greater_bids_are_pushed_back_desc() {
        let mut bids = Bids::new();
        bids.add_bid::<AggregateOrCreate>(PriceAndQuantity(1., 1));
        bids.add_bid::<AggregateOrCreate>(PriceAndQuantity(0., 1));
        assert_eq!(bids.0, [PriceAndQuantity(0., 1), PriceAndQuantity(1., 1)]);
    }

    #[test]
    fn insert_with_strategy_replace() {
        let mut bids = Bids::new();
        bids.add_bid::<ReplaceOrRemove>(PriceAndQuantity(1., 1));
        bids.add_bid::<ReplaceOrRemove>(PriceAndQuantity(1., 2));
        assert_eq!(bids.0, [PriceAndQuantity(1., 2)]);
    }

    #[test]
    fn insert_with_strategy_remove() {
        let mut bids = Bids::new();
        bids.add_bid::<ReplaceOrRemove>(PriceAndQuantity(1., 1));
        bids.add_bid::<ReplaceOrRemove>(PriceAndQuantity(1., 0));
        assert_eq!(bids.0, []);
    }

    #[test]
    fn insert_with_strategy_aggregate_or_create() {
        let mut bids = Bids::new();
        bids.add_bid::<AggregateOrCreate>(PriceAndQuantity(1., 1));
        bids.add_bid::<AggregateOrCreate>(PriceAndQuantity(1., 2));
        assert_eq!(bids.0, [PriceAndQuantity(1., 3)]);
    }
}
