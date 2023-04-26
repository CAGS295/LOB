#[cfg(feature = "serde")]
mod deserialize;

use super::ops::Updatable;
use super::{
    ops::{update_strategies::AggregateOrCreate, Update},
    PriceAndQuantity,
};
use crate::ops::{update_strategies::Strategy, BinarySearchPredicate};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::ops::{Add, Deref, DerefMut};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(PartialEq, Clone, Debug, Default)]
pub struct Asks<P = f64, Q = f64>(Vec<PriceAndQuantity<P, Q>>);

impl<P, Q> From<Vec<PriceAndQuantity<P, Q>>> for Asks<P, Q> {
    fn from(value: Vec<PriceAndQuantity<P, Q>>) -> Self {
        Self(value)
    }
}

impl<P, Q> Deref for Asks<P, Q> {
    type Target = Vec<PriceAndQuantity<P, Q>>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<P, Q> DerefMut for Asks<P, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl<P, Q> Asks<P, Q> {
    pub fn new() -> Self {
        Vec::new().into()
    }
}

impl<P, Q> Asks<P, Q>
where
    P: Clone + PartialOrd,
    Q: Clone + Add<Output = Q> + PartialEq + Default,
{
    pub fn add_ask<S>(&mut self, ask: PriceAndQuantity<P, Q>)
    where
        S: Strategy,
        Self: Update<S, Tuple<P, Q> = PriceAndQuantity<P, Q>>,
    {
        Update::insert::<P, Q>(self, ask)
    }
}

impl<P, Q> BinarySearchPredicate for Asks<P, Q> {
    fn partition_predicate<Price: PartialOrd>(lhs: &Price, rhs: &Price) -> bool {
        rhs < lhs
    }
}

impl<P, Q> Updatable for Asks<P, Q> {}

#[cfg(test)]
mod test {
    use crate::ops::update_strategies::ReplaceOrRemove;

    use super::*;

    #[test]
    fn lesser_asks_are_pushed_back_asc() {
        let mut asks = Asks::new();
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(0., 1.));
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(1., 1.));
        assert_eq!(asks.0, [PriceAndQuantity(1., 1.), PriceAndQuantity(0., 1.)]);
    }

    #[test]
    fn lesser_asks_are_pushed_back_desc() {
        let mut asks = Asks::new();
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(1., 1.));
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(0., 1.));
        assert_eq!(asks.0, [PriceAndQuantity(1., 1.), PriceAndQuantity(0., 1.)]);
    }

    #[test]
    fn insert_with_strategy_replace_or_remove() {
        let mut asks = Asks::new();
        asks.add_ask::<ReplaceOrRemove>(PriceAndQuantity(1., 1));
        asks.add_ask::<ReplaceOrRemove>(PriceAndQuantity(1., 2));
        assert_eq!(asks.0, [PriceAndQuantity(1., 2)]);
    }

    #[test]
    fn insert_with_strategy_aggregate_or_create() {
        let mut asks = Asks::new();
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(1., 1));
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(1., 2));
        assert_eq!(asks.0, [PriceAndQuantity(1., 3)]);
    }
}
