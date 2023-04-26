use std::ops::{Add, Deref, DerefMut};

pub mod update_strategies {
    pub trait Strategy {}

    impl Strategy for AggregateOrCreate {}
    impl Strategy for ReplaceOrRemove {}
    /// Inserts `Tuple` type into the vector or aggregates if it already exists.
    pub struct AggregateOrCreate;

    /// Replaces `Tuple` type into the vector or remove it if `Remove::remove()`.
    pub struct ReplaceOrRemove;
}

use update_strategies::Strategy;

///This is a trait for an aggregated insert operation.
/// It defines a GAT that takes two type parameters Price and Quantity.
pub trait Update<S: Strategy>: BinarySearchPredicate {
    type Tuple<Price, Quantity>;

    fn index<P, Q>(&self, rhs: &Self::Tuple<P, Q>) -> usize
    where
        Self: Deref<Target = Vec<Self::Tuple<P, Q>>>,
        Self::Tuple<P, Q>: AsRef<P>,
        P: PartialOrd,
    {
        self.partition_point(|value| Self::partition_predicate(value.as_ref(), rhs.as_ref()))
    }

    /// The `partition_point` method is called on the vector to find the index at which the new `Tuple` should be inserted.
    fn insert<P, Q>(prices: &mut Self, price_and_quantity: Self::Tuple<P, Q>)
    where
        Self: DerefMut<Target = Vec<Self::Tuple<P, Q>>>,
        P: PartialOrd,
        Self::Tuple<P, Q>: Clone + Add<Output = Self::Tuple<P, Q>> + AsRef<P>,
    {
        let index = prices.index(&price_and_quantity);

        let (price_and_quantity, replace) = prices.get_mut(index).map_or_else(
            || (price_and_quantity.clone(), false),
            |agg| {
                let price_and_quantity = price_and_quantity.clone();
                //same price bin
                if agg.as_ref() == price_and_quantity.as_ref() {
                    //aggregate quantity
                    (agg.clone().add(price_and_quantity), true)
                } else {
                    (price_and_quantity, false)
                }
            },
        );

        if replace {
            prices[index] = price_and_quantity;
        } else {
            prices.insert(index, price_and_quantity);
        }
    }
}

pub trait BinarySearchPredicate {
    /// Defines the ordering for the inner vector, ascending or descending.
    fn partition_predicate<P: PartialOrd>(lhs: &P, rhs: &P) -> bool;
}
