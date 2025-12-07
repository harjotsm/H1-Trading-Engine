use super::orderbook::Orderbook;
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
pub struct MachiningEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MachiningEngine {
    pub fn new() -> MachiningEngine {
        MachiningEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, trading_pair: TradingPair) {
        self.orderbooks.insert(trading_pair.clone(), Orderbook::new());
        println!("Added new market: {:?}", trading_pair.to_string());
    }
}
