use update_strategies::{AggregateOrCreate, ReplaceOrRemove};

use crate::{
    price_and_quantity::{Price, Quantity},
    PriceAndQuantity,
};
use core::ops::DerefMut;
use std::ops::Add;

pub trait PartitionPredicate {
    /// Defines an ordering for the binary search partition.
    fn partition_predicate<P: PartialOrd>(lhs: &P, rhs: &P) -> bool;
}
pub mod update_strategies {

    /// Inserts `Tuple` type into the vector or aggregates if it already exists.
    pub enum AggregateOrCreate {
        Aggregated,
        Remove,
        Create,
    }

    pub enum ReplaceOrRemove {
        Replace,
        Remove,
        Displace,
        Noop,
    }
}

// Defines how to assimilate a Level in an orderbook update.
pub trait Strategy {
    fn operation<Level: Price + Quantity>(value: &Level, level: Option<&Level>) -> Self
    where
        <Level as Quantity>::Q: Default,
        <Level as Price>::P: PartialEq,
        <Level as Quantity>::Q: PartialEq;
}

impl Strategy for AggregateOrCreate {
    fn operation<Level: Price + Quantity>(new: &Level, level: Option<&Level>) -> Self
    where
        <Level as Quantity>::Q: Default + Copy + PartialEq,
        <Level as Price>::P: PartialEq,
    {
        let p = Price::to_ref;
        let q = Quantity::to_ref;

        match level {
            Some(old) if p(old) == p(new) && *q(old) + *q(new) != Level::Q::default() => {
                Self::Aggregated
            }
            Some(old) if p(old) != p(new) => Self::Create,
            Some(_) => Self::Remove,
            None => Self::Create,
        }
    }
}

impl Strategy for ReplaceOrRemove {
    fn operation<Level: Price + Quantity>(value: &Level, level: Option<&Level>) -> Self
    where
        <Level as Quantity>::Q: Default,
        <Level as Price>::P: PartialEq,
        <Level as Quantity>::Q: PartialEq,
    {
        let p = Price::to_ref;
        let q = Quantity::to_ref;

        // found,
        //      replace if same price and not default
        //      remove if same price and default
        //      insert if new price and not default
        //      noop if new price and default
        // not found, insert if not default,

        match level {
            Some(level) if p(level) == p(value) && &Level::Q::default() != q(value) => {
                ReplaceOrRemove::Replace
            }
            Some(bin) if p(bin) == p(value) => ReplaceOrRemove::Remove,
            Some(_) if &Level::Q::default() != q(value) => ReplaceOrRemove::Displace,
            Some(_) => ReplaceOrRemove::Noop,
            None if &Level::Q::default() != q(value) => ReplaceOrRemove::Displace,
            None => ReplaceOrRemove::Noop,
        }
    }
}

pub trait Update<S: Strategy>: PartitionPredicate {
    type Level: Price + Quantity;
    type Key;

    // This method should return a mutable reference to the new level's slot, creating preemptively if not found.
    fn entry(&mut self, level_update: &Self::Level) -> (Self::Key, Option<&Self::Level>)
    where
        <Self::Level as Price>::P: PartialOrd;

    /// process the Level;
    fn process(&mut self, level_update: Self::Level)
    where
        <Self::Level as Price>::P: PartialOrd,
        <Self::Level as Quantity>::Q: Default + PartialEq,
    {
        let (key, entry) = Self::entry(self, &level_update);

        let operator = S::operation(&level_update, entry);

        self.digest_operation(operator, &key, level_update);
    }

    fn digest_operation(&mut self, operator: S, key: &Self::Key, level_update: Self::Level);
}

impl<T, P, Q> Update<ReplaceOrRemove> for T
where
    T: PartitionPredicate + DerefMut<Target = Vec<PriceAndQuantity<P, Q>>>,
    Q: Add<Q, Output = Q> + Copy,
{
    type Level = PriceAndQuantity<P, Q>;
    type Key = usize;

    fn entry(&mut self, rhs: &Self::Level) -> (Self::Key, Option<&Self::Level>)
    where
        <Self::Level as Price>::P: PartialOrd,
    {
        let index = self.partition_point(|value| {
            Self::partition_predicate(Price::to_ref(value), Price::to_ref(rhs))
        });
        (index, self.get(index))
    }

    fn digest_operation(
        &mut self,
        operator: ReplaceOrRemove,
        key: &usize,
        level_update: Self::Level,
    ) {
        match operator {
            ReplaceOrRemove::Replace => {
                self[*key] = level_update;
            }
            ReplaceOrRemove::Remove => {
                self.remove(*key);
            }
            ReplaceOrRemove::Displace => {
                self.insert(*key, level_update);
            }
            ReplaceOrRemove::Noop => {}
        }
    }
}

impl<T, P, Q> Update<AggregateOrCreate> for T
where
    T: PartitionPredicate + DerefMut<Target = Vec<PriceAndQuantity<P, Q>>>,
    Q: Add<Q, Output = Q> + Copy,
{
    type Level = PriceAndQuantity<P, Q>;
    type Key = usize;

    fn entry(&mut self, rhs: &Self::Level) -> (Self::Key, Option<&Self::Level>)
    where
        <Self::Level as Price>::P: PartialOrd,
    {
        let index = self.partition_point(|value| {
            Self::partition_predicate(Price::to_ref(value), Price::to_ref(rhs))
        });
        (index, self.get(index))
    }

    fn digest_operation(&mut self, operator: AggregateOrCreate, key: &usize, new: Self::Level) {
        match operator {
            AggregateOrCreate::Aggregated => {
                let level = self.get_mut(*key).unwrap();
                let q = level.1 + new.1;
                level.1 = q;
            }
            AggregateOrCreate::Remove => {
                self.remove(*key);
            }
            AggregateOrCreate::Create => {
                self.insert(*key, new);
            }
        }
    }
}