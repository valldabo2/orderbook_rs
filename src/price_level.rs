#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::VecDeque;

use crate::order::Order;

pub struct PriceLevel {
    pub orders: VecDeque<Order>,
    pub size: f64
}

impl PriceLevel {
    pub fn default() -> PriceLevel {
        PriceLevel{orders:VecDeque::new(), size: 0.0}
    }
    pub fn add(&mut self, o: Order) {
        self.orders.push_back(o);
        self.size += o.size;
    }
    pub fn remove(&mut self, id: &i64) {
        if let Some(pos) = self.orders.iter().position(|x| x.id == *id) {
            self.size -= self.orders.get(pos).unwrap().size;
            self.orders.remove(pos);
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn price(&self) -> Option<f64> {
        Some(self.orders.front().unwrap().price.0)
    }
}


#[cfg(test)]
mod tests {
    use crate::order::Side;

    use super::*;

    #[test]
    fn insert_remove() {
        let o = Order::new(10.0, 1.0, 0, Side::Buy);
        let mut pl = PriceLevel::default();
        assert_eq!(pl.size, 0.0);
        pl.add(o);
        assert_eq!(pl.size, o.size);
        pl.remove(&o.id);
        assert_eq!(pl.size, 0.0);

        // Removes again.
        pl.remove(&o.id);
    }
    #[test]
    fn insert_twice() {
        let o1 = Order::new(10.0, 1.0, 0, Side::Buy);
        let o2 = Order::new(10.0, 2.0, 1, Side::Buy);
        let mut pl = PriceLevel::default();
        pl.add(o1);
        pl.add(o2);
        assert_eq!(pl.size, o1.size + o2.size);
    }

    #[test]
    fn insert_remove_many_times() {
        let mut pl = PriceLevel::default();
        for i in 1..10 {
            pl.add(Order::new(10.0, 1.0, i, Side::Buy));
        }
        for i in 1..10 {
            pl.remove(&i);
        }
    }
}