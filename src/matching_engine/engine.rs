use super::orderbook::{MatchEvent, Orderbook, Price, Quantity, Side};
use rustc_hash::FxHashMap;
use std::fmt;

// Zero-Cost Identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarketId(pub u32);

/// A zero-allocation representation of a ticker symbol (up to 8 characters).
/// This keeps `TradingPair` exactly 16 bytes and entirely on the stack.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Ticker(pub [u8; 8]);

impl Ticker {
    /// Creates a new Ticker from a string slice, padding with null bytes.
    pub fn new(s: &str) -> Self {
        let mut bytes = [0u8; 8];
        let len = s.len().min(8);
        bytes[..len].copy_from_slice(&s.as_bytes()[..len]);
        Self(bytes)
    }

    /// Safely extracts the valid UTF-8 string portion for printing
    pub fn as_str(&self) -> &str {
        let len = self.0.iter().position(|&c| c == 0).unwrap_or(8);
        std::str::from_utf8(&self.0[..len]).unwrap_or("INVALID")
    }
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct TradingPair {
    base: Ticker,
    quote: Ticker,
}

impl TradingPair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base: Ticker::new(base),
            quote: Ticker::new(quote),
        }
    }
}

// Idiomatic way to handle string conversion in Rust
impl fmt::Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.base, self.quote)
    }
}

// Zero-Allocation Error Handling
#[derive(Debug, PartialEq, Eq)]
pub enum EngineError {
    MarketNotFound(MarketId),
    MarketAlreadyExists,
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::MarketNotFound(id) => write!(f, "Market ID {} does not exist", id.0),
            EngineError::MarketAlreadyExists => write!(f, "Market already exists"),
        }
    }
}
impl std::error::Error for EngineError {}

// The Engine
pub struct MatchingEngine {
    orderbooks: FxHashMap<MarketId, Orderbook>,
    pair_to_id: FxHashMap<TradingPair, MarketId>,
    next_market_id: u32,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            orderbooks: FxHashMap::default(),
            pair_to_id: FxHashMap::default(),
            next_market_id: 1, // Start at 1 (0 is often used as a null/invalid ID in binary protocols)
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) -> MarketId {
        if let Some(&id) = self.pair_to_id.get(&pair) {
            println!("Market already exists: {}", pair);
            return id;
        }

        let id = MarketId(self.next_market_id);
        self.next_market_id += 1;

        println!("Added new market: {} with ID {}", pair, id.0);

        self.orderbooks.insert(id, Orderbook::new());
        self.pair_to_id.insert(pair, id);

        id
    }

    /// Now takes a closure to route events immediately
    pub fn place_limit_order<F>(
        &mut self,
        market_id: MarketId,
        side: Side,
        price: Price,
        qty: Quantity,
        on_event: F,
    ) -> Result<(), EngineError>
    where
        F: FnMut(MatchEvent),
    {
        // Using `ok_or` avoids allocating an error string, returning our fast enum
        let orderbook = self
            .orderbooks
            .get_mut(&market_id)
            .ok_or(EngineError::MarketNotFound(market_id))?;

        // Pass the closure straight down into the execution engine
        orderbook.execute_limit_order(side, price, qty, on_event);
        
        Ok(())
    }
}