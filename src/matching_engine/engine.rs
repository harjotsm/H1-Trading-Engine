use std::collections::HashMap;
use super::orderbook::Orderbook;

pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }
}
pub struct MachiningEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}