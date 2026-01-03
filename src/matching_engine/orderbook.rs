use std::collections::BTreeMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub enum MatchEvent {
    Trade {
        maker_id: u64,
        taker_id: u64,
        price: u64,
        qty: u64,
    },
    Maker {
        id: u64,
        price: u64,
        qty: u64,
        side: BidOrAsk,
    },
}

#[derive(Debug)]
pub struct MatchResult {
    pub events: Vec<MatchEvent>,
}

#[derive(Debug)]
pub struct Orderbook {
    asks: BTreeMap<u64, Limit>,
    bids: BTreeMap<u64, Limit>,
    next_order_id: u64,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            next_order_id: 1,
        }
    }

    pub fn execute_limit_order(&mut self, side: BidOrAsk, price: u64, mut qty: u64) -> MatchResult {
        let taker_order_id = self.next_order_id;
        self.next_order_id += 1;

        let mut events = Vec::new();

        match side {
            BidOrAsk::Bid => {
                for (ask_price, limit) in self.asks.iter_mut() {
                    if *ask_price > price {
                        break;
                    }
                    if qty == 0 {
                        break;
                    }

                    let (matched, trade_events) = limit.fill(taker_order_id, qty);
                    qty -= matched;
                    events.extend(trade_events);
                }
            }
            BidOrAsk::Ask => {
                for (bid_price, limit) in self.bids.iter_mut().rev() {
                    if *bid_price < price {
                        break;
                    }
                    if qty == 0 {
                        break;
                    }

                    let (matched, trade_events) = limit.fill(taker_order_id, qty);
                    qty -= matched;
                    events.extend(trade_events);
                }
            }
        }

        if qty > 0 {
            events.push(MatchEvent::Maker {
                id: taker_order_id,
                price,
                qty,
                side,
            });

            let order = Order {
                id: taker_order_id,
                size: qty,
                bid_or_ask: side,
            };
            let target_map = match side {
                BidOrAsk::Bid => &mut self.bids,
                BidOrAsk::Ask => &mut self.asks,
            };

            target_map
                .entry(price)
                .or_insert_with(|| Limit::new(price))
                .add_order(order);
        }

        self.asks.retain(|_, l| !l.is_empty());
        self.bids.retain(|_, l| !l.is_empty());

        MatchResult { events }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: u64,
    orders: VecDeque<Order>,
}

impl Limit {
    pub fn new(price: u64) -> Limit {
        Limit {
            price,
            orders: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    fn fill(&mut self, taker_id: u64, mut qty_to_fill: u64) -> (u64, Vec<MatchEvent>) {
        let mut events = Vec::new();
        let start_qty = qty_to_fill;

        for book_order in self.orders.iter_mut() {
            if qty_to_fill == 0 {
                break;
            }

            let match_amount = std::cmp::min(qty_to_fill, book_order.size);

            book_order.size -= match_amount;
            qty_to_fill -= match_amount;

            events.push(MatchEvent::Trade {
                maker_id: book_order.id,
                taker_id: taker_id,
                price: self.price,
                qty: match_amount,
            });
        }
        self.orders.retain(|o| o.size > 0);
        (start_qty - qty_to_fill, events)
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push_back(order);
    }
}

#[derive(Debug)]
pub(crate) struct Order {
    pub size: u64,
    pub bid_or_ask: BidOrAsk,
    pub id: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_match() {
        let mut ob = Orderbook::new();
        let res1 = ob.execute_limit_order(BidOrAsk::Ask, 10, 100);
        assert!(matches!(res1.events[0], MatchEvent::Maker { .. }));
        let res2 = ob.execute_limit_order(BidOrAsk::Bid, 10, 50);

        match res2.events[0] {
            MatchEvent::Trade {
                qty,
                price,
                maker_id,
                ..
            } => {
                assert_eq!(qty, 50);
                assert_eq!(price, 10);
                assert_eq!(maker_id, 1);
            }
            _ => panic!("Expected a Trade event!"),
        }
        let limit = ob.asks.get(&10).unwrap();
        assert_eq!(limit.orders[0].size, 50);
    }

    #[test]
    fn test_price_priority() {
        let mut ob = Orderbook::new();

        ob.execute_limit_order(BidOrAsk::Ask, 200, 100);
        ob.execute_limit_order(BidOrAsk::Ask, 100, 100);

        let res = ob.execute_limit_order(BidOrAsk::Bid, 200, 50);

        match res.events[0] {
            MatchEvent::Trade { price, .. } => {
                assert_eq!(
                    price, 100,
                    "Engine chose the wrong price! Should be 100 (Best Execution)."
                );
            }
            _ => panic!("No trade occurred"),
        }
    }

    #[test]
    fn test_time_priority() {
        let mut ob = Orderbook::new();
        let res_a = ob.execute_limit_order(BidOrAsk::Ask, 100, 10);
        let id_a = if let MatchEvent::Maker { id, .. } = res_a.events[0] {
            id
        } else {
            0
        };

        let res_b = ob.execute_limit_order(BidOrAsk::Ask, 100, 10);
        let id_b = if let MatchEvent::Maker { id, .. } = res_b.events[0] {
            id
        } else {
            0
        };

        assert!(id_a < id_b, "IDs should be sequential");

        let res_buy = ob.execute_limit_order(BidOrAsk::Bid, 100, 10);

        match res_buy.events[0] {
            MatchEvent::Trade { maker_id, .. } => {
                assert_eq!(
                    maker_id, id_a,
                    "Time Priority failed! Oldest order should match first."
                );
            }
            _ => panic!("Expected Trade"),
        }
    }
}
