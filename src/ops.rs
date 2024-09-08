use crate::{
    price_and_quantity::{Price, Quantity},
    PriceAndQuantity,
};
use core::ops::{Add, Deref, DerefMut};

pub mod update_strategies {
    pub trait Strategy {
        //fn insert<I, P, Q>(prices: &mut Self, item: I);
    }

    impl Strategy for AggregateOrCreate {}
    impl Strategy for ReplaceOrRemove {}
    /// Inserts `Tuple` type into the vector or aggregates if it already exists.
    pub struct AggregateOrCreate;

    /// Replaces `Tuple` type into the vector or remove it if `Remove::remove()`.
    pub struct ReplaceOrRemove;
}

use update_strategies::{AggregateOrCreate, ReplaceOrRemove, Strategy};

///This is a trait for an aggregated insert operation.
/// It defines a GAT that takes two type parameters Price and Quantity.
/// Choose a [`Strategy`] to insert [Self::Tuple<Price, Quantity>].
pub trait Update<S: Strategy>: BinarySearchPredicate {
    type Tuple<Price, Quantity>;

    fn index<P, Q>(&self, rhs: &Self::Tuple<P, Q>) -> usize
    where
        Self: Deref<Target = Vec<Self::Tuple<P, Q>>>,
        Self::Tuple<P, Q>: Price<P = P>,
        P: PartialOrd,
    {
        self.partition_point(|value| {
            Self::partition_predicate(Price::to_ref(value), Price::to_ref(rhs))
        })
    }

    /// The `partition_point` method is called on the vector to find the index at which the new `Tuple` should be inserted.
    fn insert<P, Q>(prices: &mut Self, p_n_q: Self::Tuple<P, Q>)
    where
        Self: DerefMut<Target = Vec<Self::Tuple<P, Q>>>,
        P: PartialOrd,
        Self::Tuple<P, Q>: Clone + Price<P = P> + Quantity<Q = Q> + Add<Output = Self::Tuple<P, Q>>,
        for<'a> &'a Q: PartialEq<&'a Q>,
        Q: Default;
}

pub trait BinarySearchPredicate {
    /// Defines the ordering for the inner vector, ascending or descending.
    fn partition_predicate<P: PartialOrd>(lhs: &P, rhs: &P) -> bool;
}

impl<T> Update<ReplaceOrRemove> for T
where
    T: BinarySearchPredicate,
{
    type Tuple<Price, Quantity> = PriceAndQuantity<Price, Quantity>;

    fn insert<P, Q>(prices: &mut Self, p_n_q: Self::Tuple<P, Q>)
    where
        Self: DerefMut<Target = Vec<Self::Tuple<P, Q>>>,
        P: PartialOrd,
        Self::Tuple<P, Q>: Clone + Price<P = P> + Quantity<Q = Q> + Add<Output = Self::Tuple<P, Q>>,
        for<'a> &'a Q: PartialEq<&'a Q>,
        Q: Default,
    {
        let index = <Self as Update<ReplaceOrRemove>>::index(prices, &p_n_q);

        enum Operator {
            Replace,
            Remove,
            Insert,
            Noop,
        }

        // found,
        //      replace if same price and not default
        //      remove if same price and default
        //      insert if new price and not default
        //      noop if new price and default
        // not found, insert if not default,

        let p = Price::to_ref;
        let q = Quantity::to_ref;

        let operator = match prices.get_mut(index) {
            Some(bin) if p(bin) == p(&p_n_q) && &Q::default() != q(&p_n_q) => Operator::Replace,
            Some(bin) if p(bin) == p(&p_n_q) => Operator::Remove,
            Some(_) if &Q::default() != q(&p_n_q) => Operator::Insert,
            Some(_) => Operator::Noop,
            None if &Q::default() != q(&p_n_q) => Operator::Insert,
            None => Operator::Noop,
        };

        match operator {
            Operator::Replace => {
                prices[index] = p_n_q;
            }
            Operator::Remove => {
                prices.remove(index);
            }
            Operator::Insert => prices.insert(index, p_n_q),
            Operator::Noop => {}
        }
    }
}

impl<T> Update<AggregateOrCreate> for T
where
    T: BinarySearchPredicate,
{
    type Tuple<Price, Quantity> = PriceAndQuantity<Price, Quantity>;

    fn insert<P, Q>(prices: &mut Self, p_n_q: Self::Tuple<P, Q>)
    where
        Self: DerefMut<Target = Vec<Self::Tuple<P, Q>>>,
        P: PartialOrd,
        Self::Tuple<P, Q>: Clone + Price<P = P> + Quantity<Q = Q> + Add<Output = Self::Tuple<P, Q>>,
        for<'a> &'a Q: PartialEq<&'a Q>,
        Q: Default,
    {
        let index = <Self as Update<ReplaceOrRemove>>::index(prices, &p_n_q);

        let (p_n_q, replace) = prices.get_mut(index).map_or_else(
            || (p_n_q.clone(), false),
            |agg| {
                let p_n_q = p_n_q.clone();
                //same price bin
                if Price::to_ref(agg) == Price::to_ref(&p_n_q) {
                    //aggregate quantity
                    (agg.clone().add(p_n_q), true)
                } else {
                    (p_n_q, false)
                }
            },
        );

        if replace {
            prices[index] = p_n_q;
        } else {
            prices.insert(index, p_n_q);
        }
    }
}
