mod matching_engine;
use matching_engine::orderbook::{Orderbook, Order, BidOrAsk};
use matching_engine::engine::MachiningEngine;
use crate::matching_engine::engine::TradingPair;

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Bid, 6.5);
    orderbook.add_order(20.0, sell_order);

    // println!("{:?}", orderbook);
    let mut engine = MachiningEngine::new();
    let pair = TradingPair::new("RHM".to_string(), "EUR".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    // let pair2 = TradingPair::new("SAP".to_string(), "EUR".to_string());
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}