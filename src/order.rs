use ordered_float::OrderedFloat;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug)]
pub struct Order {
    pub price: OrderedFloat<f64>,
    pub size: f64,
    pub id: i64,
    pub side: Side,
}

impl Order {
    pub fn new(price: f64, size: f64, id: i64, side: Side) -> Order {
        Order {
            price: OrderedFloat(price),
            size,
            id,
            side,
        }
    }
}
