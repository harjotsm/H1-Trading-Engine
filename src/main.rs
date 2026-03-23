use std::collections::{BTreeMap, VecDeque};

// Newtypes for Strict Typing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Price(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side { Bid, Ask }

#[derive(Debug)]
pub enum MatchEvent {
    Trade { maker_id: OrderId, taker_id: OrderId, price: Price, qty: Quantity },
    Maker { id: OrderId, price: Price, qty: Quantity, side: Side },
}

#[derive(Debug)]
pub struct Order { pub id: OrderId, pub size: Quantity, pub side: Side }

// Orderbook Structures
#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: VecDeque<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Self { Self { price, orders: VecDeque::new() } }
    pub fn is_empty(&self) -> bool { self.orders.is_empty() }

    fn fill<F>(&mut self, taker_id: OrderId, mut qty_to_fill: Quantity, mut on_event: F) -> Quantity
    where F: FnMut(MatchEvent) {
        let start_qty = qty_to_fill.0;
        while qty_to_fill.0 > 0 {
            if let Some(mut book_order) = self.orders.pop_front() {
                let match_amount = std::cmp::min(qty_to_fill.0, book_order.size.0);
                book_order.size.0 -= match_amount;
                qty_to_fill.0 -= match_amount;

                on_event(MatchEvent::Trade {
                    maker_id: book_order.id, taker_id, price: self.price, qty: Quantity(match_amount),
                });

                if book_order.size.0 > 0 { self.orders.push_front(book_order); }
            } else { break; }
        }
        Quantity(start_qty - qty_to_fill.0)
    }
}

#[derive(Debug)]
pub struct Orderbook {
    asks: BTreeMap<Price, Limit>,
    bids: BTreeMap<Price, Limit>,
    next_order_id: OrderId,
}

impl Orderbook {
    pub fn new() -> Self {
        Self { asks: BTreeMap::new(), bids: BTreeMap::new(), next_order_id: OrderId(1) }
    }

    pub fn execute_limit_order<F>(&mut self, side: Side, price: Price, qty: Quantity, mut on_event: F)
    where F: FnMut(MatchEvent) {
        let taker_order_id = self.next_order_id;
        self.next_order_id.0 += 1;
        let mut remaining_qty = qty;
        let mut prices_to_remove = [Price(0); 8];
        let mut remove_count = 0;

        match side {
            Side::Bid => {
                for (&ask_price, limit) in self.asks.iter_mut() {
                    if ask_price > price || remaining_qty.0 == 0 { break; }
                    let matched = limit.fill(taker_order_id, remaining_qty, &mut on_event);
                    remaining_qty.0 -= matched.0;
                    if limit.is_empty() && remove_count < 8 {
                        prices_to_remove[remove_count] = ask_price;
                        remove_count += 1;
                    }
                }
                for i in 0..remove_count { self.asks.remove(&prices_to_remove[i]); }
            }
            Side::Ask => {
                for (&bid_price, limit) in self.bids.iter_mut().rev() {
                    if bid_price < price || remaining_qty.0 == 0 { break; }
                    let matched = limit.fill(taker_order_id, remaining_qty, &mut on_event);
                    remaining_qty.0 -= matched.0;
                    if limit.is_empty() && remove_count < 8 {
                        prices_to_remove[remove_count] = bid_price;
                        remove_count += 1;
                    }
                }
                for i in 0..remove_count { self.bids.remove(&prices_to_remove[i]); }
            }
        }

        if remaining_qty.0 > 0 {
            on_event(MatchEvent::Maker { id: taker_order_id, price, qty: remaining_qty, side });
            let target_map = match side { Side::Bid => &mut self.bids, Side::Ask => &mut self.asks };
            target_map.entry(price).or_insert_with(|| Limit::new(price))
                .orders.push_back(Order { id: taker_order_id, size: remaining_qty, side });
        }
    }
}

fn main() {
    let mut ob = Orderbook::new();

    ob.execute_limit_order(Side::Ask, Price(100), Quantity(10), |event| {
        println!("Setup Event: {:?}", event);
    });

    println!("--- Executing aggressive Bid ---");

    ob.execute_limit_order(Side::Bid, Price(100), Quantity(5), |event| {
        match event {
            MatchEvent::Trade { maker_id, taker_id, price, qty } => {
                println!(
                    "TRADE! Taker {} matched with Maker {} for {} units at ${}",
                    taker_id.0, maker_id.0, qty.0, price.0
                );
            }
            MatchEvent::Maker { .. } => {
                println!("Order added to book.");
            }
        }
    });
}