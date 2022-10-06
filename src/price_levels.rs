#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::BTreeMap;
use std::collections::HashMap;
use ordered_float::OrderedFloat;

use crate::order::Order;
use crate::price_level::PriceLevel;

pub struct PriceLevels {
    pub levels: BTreeMap<OrderedFloat<f64>, PriceLevel>,
    orders: HashMap<i64, Order>,
    ask: bool
}

impl PriceLevels {
    pub fn new(ask: bool) -> PriceLevels {
        PriceLevels{ levels: BTreeMap::new(), orders: HashMap::new(), ask}
    }
    pub fn insert(&mut self, o: Order) {
        if self.levels.contains_key(&o.price){
            let pl = self.levels.get_mut(&o.price).unwrap();
            pl.add(o);
        } else {
            let mut pl = PriceLevel::default();
            pl.add(o);
            self.levels.insert(o.price, pl);
        }
        self.orders.insert(o.id, o);
    }

    pub fn remove_order(&mut self, id: &i64) {
        let orders = &mut self.orders;
        let levels = &mut self.levels;

        let o = orders.get(id).unwrap();
        let pl = levels.get_mut(&o.price).unwrap();
        pl.remove(id);
        if pl.is_empty() {
            levels.remove(&o.price);
        }
        orders.remove(id);
    }

    pub fn update_order(&mut self, id: &i64, new_size: f64) {
        let orders = &mut self.orders;
        let levels = &mut self.levels;

        let o = orders.get_mut(id).unwrap();
        let pl = levels.get_mut(&o.price).unwrap();

        let size_change = new_size - o.size;
        pl.size += size_change;
        o.size = new_size;
        
    }

    fn _best_price(&self) -> Option<&OrderedFloat<f64>> {
        if !self.levels.is_empty() {
            let kv;
            if self.ask {
                kv = self.levels.first_key_value();
            } else {
                kv = self.levels.last_key_value();
            }
            Some(kv.unwrap().0)
        } else {
            None
        }
    }

    pub fn best_price(&self) -> Option<f64> {
        let bp = self._best_price();
        match bp {
            Some(bp) => return Some(bp.clone().0),
            None => return None
        }

    }

    pub fn best_level(&self) -> Option<&PriceLevel> {
        let bp = self._best_price();
        match bp {
            Some(bp) => return Some(self.levels.get(bp).unwrap()),
            None => None
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::order::Side;

    use super::*;

    #[test]
    fn insert_ask() {
        let o1 = Order::new(10.0, 1.0, 0, Side::Buy);
        let mut pls = PriceLevels::new(true);
        pls.insert(o1);
        assert_eq!(pls.best_price().unwrap(), 10.0);
        let o2 = Order::new(9.0, 2.0, 1, Side::Buy);
        pls.insert(o2);
        assert_eq!(pls.best_price().unwrap(), 9.0);
    }
    #[test]
    fn insert_bid() {
        let o1 = Order::new(9.0, 1.0, 0, Side::Buy);
        let o2 = Order::new(10.0, 1.0, 1,Side::Buy);
        let mut pls = PriceLevels::new(false);
        pls.insert(o1);
        pls.insert(o2);
        assert_eq!(pls.best_price().unwrap(), 10.0);
    }
}
