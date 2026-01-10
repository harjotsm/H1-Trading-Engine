use H1_Trading_Engine::matching_engine::engine::{MatchingEngine, TradingPair};
use H1_Trading_Engine::matching_engine::orderbook::BidOrAsk;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use std::hint::black_box;

fn benchmark_placing_orders(c: &mut Criterion) {
    let mut group = c.benchmark_group("Orderbook Throughput");
    group.sample_size(50);

    group.bench_function("place_random_orders", |b| {
        let mut engine = MatchingEngine::new();
        let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
        let market_id = engine.add_new_market(pair);

        let mut rng = rand::rng();
        let orders: Vec<(BidOrAsk, u64, u64)> = (0..1000)
            .map(|_| {
                let side = if rng.random_bool(0.5) {
                    BidOrAsk::Bid
                } else {
                    BidOrAsk::Ask
                };
                let price = rng.random_range(100..200);
                let qty = rng.random_range(1..10);
                (side, price, qty)
            })
            .collect();

        b.iter(|| {
            for (side, price, qty) in &orders {
                let _ = black_box(engine.place_limit_order(market_id, *side, *price, *qty));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_placing_orders);
criterion_main!(benches);
