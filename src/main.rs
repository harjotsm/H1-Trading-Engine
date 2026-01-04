mod matching_engine;
use H1_Trading_Engine::matching_engine::engine::{MatchingEngine, TradingPair};
use H1_Trading_Engine::matching_engine::orderbook::{BidOrAsk, MatchEvent};

fn main() {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    println!("--- \u{1F680} TRADING ENGINE STARTED: BTC_USD ---\n");

    println!("1. Alice places SELL Order (2 BTC @ $50,000)...");
    let res_alice = engine
        .place_limit_order(pair.clone(), BidOrAsk::Ask, 50_000, 2)
        .unwrap();
    print_events(res_alice.events);

    println!("\n2. Bob places BUY Order (1 BTC @ $50,000)...");
    let res_bob = engine
        .place_limit_order(pair.clone(), BidOrAsk::Bid, 50_000, 1)
        .unwrap();
    print_events(res_bob.events);

    println!("\n3. Charlie places BUY Order (2 BTC @ $50,000)...");
    let res_charlie = engine
        .place_limit_order(pair.clone(), BidOrAsk::Bid, 50_000, 2)
        .unwrap();
    print_events(res_charlie.events);
}

fn print_events(events: Vec<MatchEvent>) {
    for event in events {
        match event {
            MatchEvent::Trade {
                maker_id,
                taker_id,
                price,
                qty,
            } => {
                println!(
                    "   \u{26A1} TRADE EXECUTED: Maker #{} sold to Taker #{} -> {} units @ ${}",
                    maker_id, taker_id, qty, price
                );
            }
            MatchEvent::Maker {
                id,
                price,
                qty,
                side,
            } => {
                println!(
                    "   \u{1F4DD} ORDER PLACED: Order #{} ({:?}) is now resting in book -> {} units @ ${}",
                    id, side, qty, price
                );
            }
        }
    }
}
