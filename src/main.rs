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

    // println!("{:?}", orderbook);
    let mut engine = MachiningEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USDT".to_string());
    engine.add_new_market(pair);
}