#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[cfg_attr(feature = "serde", derive(Deserialize), derive(Serialize))]
#[derive(Clone, Debug, PartialEq, Default)]
///Careful, this struct manually implements Add which in this context, is not commutative. It adds quantity while copying the rhs' price.
/// If prices are not equal, it is also not associative; Adding quantities from different price levels is not a sound operation.
pub struct PriceAndQuantity<P, Q>(pub P, pub Q);

impl<P, Q> AsRef<P> for PriceAndQuantity<P, Q> {
    fn as_ref(&self) -> &P {
        &self.0
    }
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
