#[cfg(feature = "serde")]
mod deserializer;

use crate::ops::{
    update_strategies::{AggregateOrCreate, Strategy},
    BinarySearchPredicate,
};

use super::{ops::Update, PriceAndQuantity};
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
    Q: Clone + Add<Output = Q>,
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

impl<P, Q> Update<AggregateOrCreate> for Bids<P, Q> {
    type Tuple<Price, Quantity> = PriceAndQuantity<Price, Quantity>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn greater_bids_are_pushed_back_asc() {
        let mut bids = Bids::new();
        bids.add_bid(PriceAndQuantity(0., 0));
        bids.add_bid(PriceAndQuantity(1., 0));
        assert_eq!(bids.0, [PriceAndQuantity(0., 0), PriceAndQuantity(1., 0)]);
    }

    #[test]
    fn greater_bids_are_pushed_back_desc() {
        let mut bids = Bids::new();
        bids.add_bid(PriceAndQuantity(1., 0));
        bids.add_bid(PriceAndQuantity(0., 0));
        assert_eq!(bids.0, [PriceAndQuantity(0., 0), PriceAndQuantity(1., 0)]);
    }
}
