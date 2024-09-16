#[cfg(feature = "serde")]
mod deserialize;

use crate::ops::{PartitionPredicate, Strategy};

use super::{
    ops::{update_strategies::AggregateOrCreate, Update},
    PriceAndQuantity,
};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::fmt::Display;
use std::ops::{Add, Deref, DerefMut};

#[cfg_attr(feature = "codec", derive(crate::Encode, crate::Decode))]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(PartialEq, Clone, Debug, Default)]
pub struct Asks<P = f64, Q = f64>(Vec<PriceAndQuantity<P, Q>>);

impl<P, Q> From<Vec<PriceAndQuantity<P, Q>>> for Asks<P, Q> {
    fn from(value: Vec<PriceAndQuantity<P, Q>>) -> Self {
        Self(value)
    }
}

impl<P, Q> Display for Asks<P, Q>
where
    P: Display,
    Q: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, p) in self.0.iter().enumerate() {
            write!(f, "{}", p)?;
            if i < self.0.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")?;
        Ok(())
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
    Q: Add<Q, Output = Q> + Copy,
{
    pub fn add_ask<S>(&mut self, ask: PriceAndQuantity<P, Q>)
    where
        S: Strategy,
        Self: Update<S, Level = PriceAndQuantity<P, Q>>,
    {
        Update::process(self, ask)
    }
}

impl<P, Q> PartitionPredicate for Asks<P, Q> {
    fn partition_predicate<Price: PartialOrd>(lhs: &Price, rhs: &Price) -> bool {
        rhs < lhs
    }
}

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
    fn insert_with_strategy_replace() {
        let mut asks = Asks::new();
        asks.add_ask::<ReplaceOrRemove>(PriceAndQuantity(1., 1));
        asks.add_ask::<ReplaceOrRemove>(PriceAndQuantity(1., 2));
        assert_eq!(asks.0, [PriceAndQuantity(1., 2)]);
    }

    #[test]
    fn insert_with_strategy_remove() {
        let mut asks = Asks::new();
        asks.add_ask::<ReplaceOrRemove>(PriceAndQuantity(1., 1));
        asks.add_ask::<ReplaceOrRemove>(PriceAndQuantity(1., 0));
        assert_eq!(asks.0, []);
    }

    #[test]
    fn insert_with_strategy_aggregate_or_create() {
        let mut asks = Asks::new();
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(1., 1));
        asks.add_ask::<AggregateOrCreate>(PriceAndQuantity(1., 2));
        assert_eq!(asks.0, [PriceAndQuantity(1., 3)]);
    }
}
