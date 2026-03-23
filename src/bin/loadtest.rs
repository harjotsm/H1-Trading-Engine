use std::time::Instant;
use rand::Rng;

use H1_Trading_Engine::matching_engine::engine::{MatchingEngine, TradingPair};
use H1_Trading_Engine::matching_engine::orderbook::{MatchEvent, Price, Quantity, Side};

struct TestOrder {
    side: Side,
    price: Price,
    qty: Quantity,
}

fn main() {
    let mut engine = MatchingEngine::new();
    
    // Using new zero-allocation Ticker strings
    let pair = TradingPair::new("BTC", "USD");
    let market_id = engine.add_new_market(pair);
    
    let mut rng = rand::rng();
    let num_orders = 1_000_000;

    println!("1. Generating {} random orders...", num_orders);
    
    // Pre-allocate the vector so we don't benchmark memory allocation
    let mut orders = Vec::with_capacity(num_orders);
    
    for _ in 0..num_orders {
        let side = if rng.random_bool(0.5) {
            Side::Bid
        } else {
            Side::Ask
        };
        // Wrapping primitives in our Newtypes
        let price = Price(rng.random_range(90..110));
        let qty = Quantity(rng.random_range(1..100));
        
        orders.push(TestOrder { side, price, qty });
    }

    println!("2. Starting execution engine benchmark...");
    
    // Track how many actual trades occur for our sanity check
    let mut total_trades = 0;

    let start = Instant::now();

    for order in orders {
        let _ = engine.place_limit_order(market_id, order.side, order.price, order.qty, |event| {
            // This closure is our zero-allocation callback. 
            // In a benchmark, we want to do as little work here as possible.
            if let MatchEvent::Trade { .. } = event {
                total_trades += 1;
            }
        });
    }

    let elapsed = start.elapsed();
    
    // Calculate metrics
    let elapsed_secs = elapsed.as_secs_f64();
    let orders_per_second = (num_orders as f64) / elapsed_secs;

    println!("--- Benchmark Results ---");
    println!("Total Time:        {:?}", elapsed);
    println!("Throughput:        {:x^} orders/sec", orders_per_second.round() as u64);
    println!("Trades Executed:   {}", total_trades);
    println!("-------------------------");
}