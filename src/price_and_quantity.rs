#[cfg(feature = "serde")]
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
use std::{marker::PhantomData, ops::Add, str::FromStr};

#[cfg_attr(feature = "serde", derive(Deserialize), derive(Serialize))]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
///Careful, this struct manually implements [Add] which in this context, is not commutative. It adds quantity while copying the rhs' price.
/// If prices are not equal, it is also not associative; Adding quantities from different price levels is not a sound operation.
pub struct PriceAndQuantity<P, Q>(
    #[serde(deserialize_with = "de_from_str")]
    #[serde(bound(deserialize = "P: FromStr, P::Err: Display"))]
    pub P,
    #[serde(deserialize_with = "de_from_str")]
    #[serde(bound(deserialize = "Q: FromStr, Q::Err: Display"))]
    pub Q,
);

pub trait Price {
    type P;
    fn to_ref(&self) -> &Self::P;
}

pub trait Quantity {
    type Q;
    fn to_ref(&self) -> &Self::Q;
}

impl<P, Q> Price for PriceAndQuantity<P, Q> {
    type P = P;

    fn to_ref(&self) -> &Self::P {
        &self.0
    }
}

impl<P, Q> Quantity for PriceAndQuantity<P, Q> {
    type Q = Q;

    fn to_ref(&self) -> &Self::Q {
        &self.1
    }
}

fn de_from_str<'de, D, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T::Err: Display,
{
    struct NumberFromStr<V>(PhantomData<V>);

    impl<'de, Error: Display, V: FromStr<Err = Error>> Visitor<'de> for NumberFromStr<V> {
        type Value = V;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(r#"a string containing "float" or "number""#)
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Self::Value::from_str(s).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_str(NumberFromStr::<T>(PhantomData))
}

///Not commutative. It adds quantity while copying the rhs' price. Also, not associative if prices differ.
impl<P, Q: Add<Output = Q>> Add for PriceAndQuantity<P, Q> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0, self.1 + rhs.1)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn add_quantity_only() {
        assert_eq!(
            PriceAndQuantity(1., 1) + PriceAndQuantity(1., 1),
            PriceAndQuantity(1., 2)
        );
    }
}
