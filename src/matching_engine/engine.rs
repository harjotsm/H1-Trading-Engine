use super::orderbook::{BidOrAsk, MatchResult, Orderbook};
use rustc_hash::FxHashMap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarketId(pub u32);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: FxHashMap<MarketId, Orderbook>,
    pair_to_id: HashMap<TradingPair, MarketId>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: FxHashMap::with_capacity_and_hasher(1024, Default::default()),
            pair_to_id: HashMap::with_capacity(1024),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) -> MarketId {
        if let Some(&id) = self.pair_to_id.get(&pair) {
            println!("Market already exists: {:?}", pair.to_string());
            return id;
        }

        let id = MarketId(self.orderbooks.len() as u32);

        println!("Added new market: {:?} with ID {:?}", pair.to_string(), id);

        self.orderbooks.insert(id, Orderbook::new());
        self.pair_to_id.insert(pair, id);

        id
    }

    pub fn place_limit_order(
        &mut self,
        market_id: MarketId,
        side: BidOrAsk,
        price: u64,
        qty: u64,
    ) -> Result<MatchResult, String> {
        match self.orderbooks.get_mut(&market_id) {
            Some(orderbook) => {
                let result = orderbook.execute_limit_order(side, price, qty);
                Ok(result)
            }
            None => Err(format!("Market ID {:?} does not exist", market_id)),
        }
    }
}
