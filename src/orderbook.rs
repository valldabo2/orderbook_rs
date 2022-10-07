#![allow(dead_code)]
#![allow(unused_variables)]

use ordered_float::OrderedFloat;
use std::collections::{HashMap, VecDeque};

use crate::{
    order::{Order, Side},
    price_level::PriceLevel,
    price_levels::PriceLevels,
};

pub struct OrderBook {
    bid_levels: PriceLevels,
    ask_levels: PriceLevels,
    orders: HashMap<i64, Order>,
}

fn match_order(o: &Order, pls: &mut PriceLevels) -> Matches {
    let mut ma = Matches {
        matches: VecDeque::new(),
        order: o.clone(),
    };
    for (p, pl) in pls.levels.iter() {
        if aggressive(&ma.order, &p) {
            for nm in pl.orders.iter() {
                if ma.order.size >= nm.size {
                    // Full Match
                    let m = Match {
                        order: *nm,
                        size: nm.size,
                    };
                    ma.matches.push_back(m);
                    ma.order.size -= nm.size;
                } else {
                    // Partial Match
                    let m = Match {
                        order: *nm,
                        size: ma.order.size,
                    };
                    ma.matches.push_back(m);
                    ma.order.size = 0.0;
                    return ma
                }
            }
        } else {
            return ma
        }
    }
    return ma;
}

struct Match {
    order: Order,
    size: f64,
}
pub struct Matches {
    matches: VecDeque<Match>,
    order: Order,
}

impl Matches {
    fn total_size(&self) -> f64 {
        self.matches.iter().map(|x| x.size).sum()
    }
}

fn aggressive(o: &Order, p: &OrderedFloat<f64>) -> bool {
    if o.side == Side::Buy {
        o.price.0 >= p.0
    } else {
        o.price.0 <= p.0
    }
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bid_levels: PriceLevels::new(false),
            ask_levels: PriceLevels::new(true),
            orders: HashMap::new(),
        }
    }

    pub fn cancel(&mut self, id: &i64) {
        if self.orders.contains_key(id) {
            let o = self.orders.get(id).unwrap();
            if o.side == Side::Buy {
                self.bid_levels.remove_order(id);
            } else {
                self.ask_levels.remove_order(id);
            }
            self.orders.remove(id);
        }
    }

    pub fn update(&mut self, id: &i64, new_size: f64) {
        if self.orders.contains_key(id) {
            let o = self.orders.get(id).unwrap();
            if o.side == Side::Buy {
                self.bid_levels.update_order(id, new_size);
            } else {
                self.ask_levels.update_order(id, new_size);
            }
        }
    }

    pub fn place(&mut self, o: &Order) -> Matches {
        let mut pls: &mut PriceLevels;
        if o.side == Side::Buy {
            pls = &mut self.ask_levels;
        } else {
            pls = &mut self.bid_levels;
        }

        let mo = match_order(o, pls);

        let orders = &mut self.orders;
        // Updates orders based on matches
        for m in mo.matches.iter() {
            if m.size == m.order.size {
                // Full Match
                pls.remove_order(&m.order.id);
                orders.remove(&m.order.id);
            } else {
                // Partial Match
                let new_size = m.order.size - m.size;
                pls.update_order(&m.order.id, new_size)
            }
        }

        if mo.order.size > 0.0 {
            orders.insert(mo.order.id, mo.order);

            // We insert it on the other side
            if o.side == Side::Buy {
                pls = &mut self.bid_levels;
            } else {
                pls = &mut self.ask_levels;
            }
            pls.insert(mo.order);
        }
        mo
    }

    fn bid(&mut self) -> Option<&PriceLevel> {
        self.bid_levels.best_level()
    }

    fn ask(&mut self) -> Option<&PriceLevel> {
        self.ask_levels.best_level()
    }
}

#[cfg(test)]
mod tests {
    use crate::order::Side;

    use super::*;

    #[test]
    fn place() {
        // Places a buy order
        let o = Order::new(9.0, 1.0, 0, Side::Buy);
        let mut ob = OrderBook::new();
        ob.place(&o);
        assert_eq!(ob.bid().unwrap().price().unwrap(), 9.0);
        assert_eq!(ob.bid().unwrap().size, 1.0);

        // Places an order that matches and inserts a sell order.
        let o2 = Order::new(9.0, 3.0, 0, Side::Sell);
        let mo = ob.place(&o2);
        assert_eq!(mo.matches.len(), 1);
        assert_eq!(ob.ask().unwrap().price().unwrap(), 9.0);
        assert_eq!(ob.ask().unwrap().size, 2.0);
    }

    #[test]
    fn place_double_match() {
        let mut ob = OrderBook::new();
        // Places a buy order
        ob.place(&Order::new(9.0, 1.0, 0, Side::Buy));
        ob.place(&Order::new(10.0, 2.0, 1, Side::Buy));
        // Places an order that matches
        let mo = ob.place(&Order::new(9.0, 3.0, 2, Side::Sell));
        assert_eq!(mo.matches.len(), 2);
        assert_eq!(mo.total_size(), 3.0);
    }

    #[test]
    fn cancel() {
        let mut ob = OrderBook::new();
        let o = Order::new(9.0, 1.0, 0, Side::Buy);
        ob.place(&o);

        let o = Order::new(10.0, 2.0, 1, Side::Buy);
        ob.place(&o);

        ob.cancel(&1);
        assert_eq!(ob.bid().unwrap().price().unwrap(), 9.0);
        assert_eq!(ob.bid().unwrap().size, 1.0);
    }

    #[test]
    fn update() {
        let mut ob = OrderBook::new();
        let o = Order::new(9.0, 1.0, 0, Side::Buy);
        ob.place(&o);
        ob.update(&0, 2.0);
        assert_eq!(ob.bid().unwrap().size, 2.0);
    }
}
