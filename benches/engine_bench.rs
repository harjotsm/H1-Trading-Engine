use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use rand::Rng;

use H1_Trading_Engine::matching_engine::engine::{MatchingEngine, TradingPair};
use H1_Trading_Engine::matching_engine::orderbook::{Price, Quantity, Side};

fn benchmark_placing_orders(c: &mut Criterion) {
    let mut group = c.benchmark_group("Orderbook Throughput");
    group.sample_size(50); 

    group.bench_function("place_1000_random_orders", |b| {
        let mut rng = rand::rng();
        
        // Pre-generate the workload outside the timed loop
        let orders: Vec<(Side, Price, Quantity)> = (0..1000)
            .map(|_| {
                let side = if rng.random_bool(0.5) {
                    Side::Bid
                } else {
                    Side::Ask
                };
                let price = Price(rng.random_range(100..200));
                let qty = Quantity(rng.random_range(1..10));
                (side, price, qty)
            })
            .collect();

        // iter_batched to prevent state accumulation
        b.iter_batched(
            || {
                // create a fresh engine for every single iteration.
                let mut engine = MatchingEngine::new();
                let pair = TradingPair::new("BTC", "USD"); 
                let market_id = engine.add_new_market(pair);
                (engine, market_id)
            },
            |(mut engine, market_id)| {
                // EXECUTION PHASE: This IS timed.
                for (side, price, qty) in &orders {
                    // Pipe events into the black_box so the compiler doesn't optimize 
                    // the execution loop away entirely.
                    let _ = engine.place_limit_order(market_id, *side, *price, *qty, |event| {
                        black_box(event); 
                    });
                }
            },
            // Tells Criterion how to manage the setup memory
            BatchSize::SmallInput, 
        );
    });

    group.finish();
}

criterion_group!(benches, benchmark_placing_orders);
criterion_main!(benches);