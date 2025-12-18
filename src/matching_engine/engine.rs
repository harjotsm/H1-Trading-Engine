use super::orderbook::{Order, Orderbook};
use std::collections::HashMap;
use rust_decimal::Decimal;

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

    //@TODO use enums/defining structs that includes enums and string
    pub fn add_new_market(&mut self, trading_pair: TradingPair) {
        println!("Added new market: {:?}", trading_pair.to_string());
        self.orderbooks.insert(trading_pair, Orderbook::new());

    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("Placed new limit order: {:?}", price);
                Ok(())
            }
            None => {
                Err(format!("Trading pair {:?} not found", pair.to_string()))
            }
        }
    }
}
