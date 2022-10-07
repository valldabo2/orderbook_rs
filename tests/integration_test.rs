use orderbook_rs::Order;
use orderbook_rs::OrderBook;
use orderbook_rs::Side;

use ordered_float::OrderedFloat;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use std::collections::VecDeque;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[test]
fn test_speed() {
    let mut ob = OrderBook::new();
    let mut orders: VecDeque<Order> = VecDeque::new();
    let mut rng = thread_rng();
    let sides = [Side::Buy, Side::Sell];
    let price_range = 80..120;
    let size_range = 1..10;
    let n_orders = 1000_000;
    for i in 0..n_orders {
        let p = OrderedFloat(rng.gen_range(price_range.clone()) as f64);
        let s = rng.gen_range(size_range.clone()) as f64;
        let side = sides.choose(&mut rng).unwrap();
        orders.push_back(Order {
            price: p,
            size: s,
            side: *side,
            id: i,
        });
    }

    sleep(Duration::from_secs(2));

    let start = Instant::now();
    for o in orders.iter() {
        ob.place(o);
    }
    let duration = start.elapsed();
    let orders_per_second = n_orders as f64 / (duration.as_millis() as f64 / 1_000.0);
    println!(
        "Orders:{} in {:?}, orders/sec: {:?}",
        n_orders, duration, orders_per_second
    );
}
