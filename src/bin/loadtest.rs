use H1_Trading_Engine::matching_engine::engine::{MatchingEngine, TradingPair};
use H1_Trading_Engine::matching_engine::orderbook::BidOrAsk;
use rand::Rng;

fn main() {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    let market_id = engine.add_new_market(pair);
    let mut rng = rand::rng();

    println!("Starting load test (1 Million Orders)...");

    for _ in 0..1_000_000 {
        let side = if rng.random_bool(0.5) {
            BidOrAsk::Bid
        } else {
            BidOrAsk::Ask
        };
        let price = rng.random_range(90..110);
        let qty = rng.random_range(1..100);
        let _ = engine.place_limit_order(market_id, side, price, qty);
    }
    println!("Done.");
}
