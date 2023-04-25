pub mod asks;
pub mod bids;
pub mod limit_order_book;
mod ops;
mod price_and_quantity;

use asks::Asks;
use bids::Bids;
pub use limit_order_book::{DepthUpdate, LimitOrderBook};
pub use price_and_quantity::PriceAndQuantity;

#[derive(Clone, Debug, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order<P, Q> {
    price_quantity: PriceAndQuantity<P, Q>,
    order_type: OrderType,
}
