use super::{ops::AggregatedInsert, PriceAndQuantity};
use std::ops::{Add, Deref, DerefMut};

#[derive(PartialEq, Clone, Debug)]
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
    Q: Clone + Add<Output = Q>,
{
    pub fn add_ask(&mut self, ask: PriceAndQuantity<P, Q>) {
        AggregatedInsert::insert(self, ask)
    }
}

impl<P, Q> AggregatedInsert for Asks<P, Q> {
    type Tuple<Price, Quantity> = PriceAndQuantity<Price, Quantity>;
    fn partition_predicate<Price: PartialOrd>(lhs: &Price, rhs: &Price) -> bool {
        rhs < lhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lesser_asks_are_pushed_back_asc() {
        let mut asks = Asks::new();
        asks.add_ask(PriceAndQuantity(0., 0.));
        asks.add_ask(PriceAndQuantity(1., 0.));
        assert_eq!(asks.0, [PriceAndQuantity(1., 0.), PriceAndQuantity(0., 0.)]);
    }

    #[test]
    fn lesser_asks_are_pushed_back_desc() {
        let mut asks = Asks::new();
        asks.add_ask(PriceAndQuantity(1., 0.));
        asks.add_ask(PriceAndQuantity(0., 0.));
        assert_eq!(asks.0, [PriceAndQuantity(1., 0.), PriceAndQuantity(0., 0.)]);
    }
}
