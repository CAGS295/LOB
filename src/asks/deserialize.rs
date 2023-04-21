use std::ops::Add;

use super::AggregatedInsert;
use super::Asks;
use super::PriceAndQuantity;
use serde::{Deserialize, Deserializer as DeserializerT};

// Naive deserialization, consider implementing a visitor to stream json values instead of deseralizing into a vec and draining into the output type.
impl<'de, P, Q> Deserialize<'de> for Asks<P, Q>
where
    P: Deserialize<'de> + PartialOrd + Clone,
    Q: Deserialize<'de> + Add<Output = Q> + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: DeserializerT<'de>,
    {
        let mut prices: Vec<PriceAndQuantity<P, Q>> = Deserialize::deserialize(deserializer)?;
        let mut asks = Asks::new();
        for i in prices.drain(..) {
            AggregatedInsert::insert(&mut asks, i)
        }
        Ok(asks)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let asks: Asks<f64, i32> = vec![PriceAndQuantity(1., 2)].into();
        let expected: Asks<f64, i32> = serde_json::from_str("[[1.0,2]]").unwrap();
        assert_eq!(asks, expected);
    }
}
