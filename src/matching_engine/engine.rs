use super::orderbook::{BidOrAsk, MatchResult, Orderbook};
use std::collections::HashMap;

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
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        println!("Added new market: {:?}", pair.to_string());
        self.orderbooks.insert(pair, Orderbook::new());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        side: BidOrAsk,
        price: u64,
        qty: u64,
    ) -> Result<MatchResult, String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                let result = orderbook.execute_limit_order(side, price, qty);
                Ok(result)
            }
            None => Err(format!(
                "Market {:?} does not exist. Create it first!",
                pair.to_string()
            )),
        }
    }
}
