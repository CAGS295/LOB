use super::Bids;
use super::PriceAndQuantity;
use super::Update;
use crate::ops::update_strategies::AggregateOrCreate;
use serde::{Deserialize, Deserializer as DeserializerT};
use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

/// Naive deserialization, consider implementing a visitor to stream json values instead of deseralizing into a vec and draining into the output type.
/// It uses the [AggregateOrCreate](super::AggregateOrCreate) strategy to fill the vec.
impl<'de, P, Q> Deserialize<'de> for Bids<P, Q>
where
    P: Deserialize<'de> + FromStr,
    P::Err: Display,
    Q: Deserialize<'de> + FromStr,
    Q::Err: Display,
    P: PartialOrd,
    Q: Add<Output = Q> + Default + PartialEq + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: DeserializerT<'de>,
    {
        let mut prices: Vec<PriceAndQuantity<P, Q>> = Deserialize::deserialize(deserializer)?;
        let mut bids = Bids::new();
        for i in prices.drain(..) {
            Update::<AggregateOrCreate>::process(&mut bids, i)
        }
        Ok(bids)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let expected: Bids<f64, i32> = serde_json::from_str(r#"[["1.0","2"]]"#).unwrap();
        let bids: Bids<f64, i32> = vec![PriceAndQuantity(1., 2)].into();
        assert_eq!(bids, expected);
    }
}
