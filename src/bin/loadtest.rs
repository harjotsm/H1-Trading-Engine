use H1_Trading_Engine::matching_engine::engine::{MatchingEngine, TradingPair};
use H1_Trading_Engine::matching_engine::orderbook::BidOrAsk;
use rand::Rng;

fn main() {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let mut rng = rand::thread_rng();

    println!("Starting load test (1 Million Orders)...");

    for _ in 0..1_000_000 {
        let side = if rng.gen_bool(0.5) { BidOrAsk::Bid } else { BidOrAsk::Ask };
        let price = rng.gen_range(90..110);
        let qty = rng.gen_range(1..100);

        let _ = engine.place_limit_order(pair.clone(), side, price, qty);
    }

    println!("Done.");
}