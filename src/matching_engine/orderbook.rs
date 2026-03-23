use std::collections::{BTreeMap, VecDeque};

// Newtypes for Strict Typing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Price(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

// Zero-Allocation Event Structures
#[derive(Debug)]
pub enum MatchEvent {
    Trade {
        maker_id: OrderId,
        taker_id: OrderId,
        price: Price,
        qty: Quantity,
    },
    Maker {
        id: OrderId,
        price: Price,
        qty: Quantity,
        side: Side,
    },
}

#[derive(Debug)]
pub struct Order {
    pub id: OrderId,
    pub size: Quantity,
    pub side: Side,
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: VecDeque<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Self {
        Self {
            price,
            orders: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    /// Fills orders at this price level, firing the callback immediately 
    /// instead of allocating a Vec of events.
    fn fill<F>(&mut self, taker_id: OrderId, mut qty_to_fill: Quantity, mut on_event: F) -> Quantity
    where
        F: FnMut(MatchEvent),
    {
        let start_qty = qty_to_fill.0;

        while qty_to_fill.0 > 0 {
            if let Some(mut book_order) = self.orders.pop_front() {
                let match_amount = std::cmp::min(qty_to_fill.0, book_order.size.0);

                book_order.size.0 -= match_amount;
                qty_to_fill.0 -= match_amount;

                // Fire the event instantly. No memory allocation.
                on_event(MatchEvent::Trade {
                    maker_id: book_order.id,
                    taker_id,
                    price: self.price,
                    qty: Quantity(match_amount),
                });

                if book_order.size.0 > 0 {
                    self.orders.push_front(book_order);
                }
            } else {
                break;
            }
        }

        Quantity(start_qty - qty_to_fill.0)
    }
}

// Core Engine
#[derive(Debug)]
pub struct Orderbook {
    asks: BTreeMap<Price, Limit>,
    bids: BTreeMap<Price, Limit>,
    next_order_id: OrderId,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            next_order_id: OrderId(1),
        }
    }

    /// Executes an order using a zero-allocation callback pattern.
    pub fn execute_limit_order<F>(
        &mut self,
        side: Side,
        price: Price,
        qty: Quantity,
        mut on_event: F,
    ) where
        // This trait bound means we accept any closure that takes a MatchEvent
        F: FnMut(MatchEvent), 
    {
        let taker_order_id = self.next_order_id;
        self.next_order_id.0 += 1;

        let mut remaining_qty = qty;

        // Stack-allocated array for garbage collection. 
        // 8 is usually plenty for a single order sweep. 
        // In production, you might use arrayvec::ArrayVec<[Price; 32]>.
        let mut prices_to_remove = [Price(0); 8];
        let mut remove_count = 0;

        match side {
            Side::Bid => {
                for (&ask_price, limit) in self.asks.iter_mut() {
                    if ask_price > price || remaining_qty.0 == 0 {
                        break;
                    }

                    let matched = limit.fill(taker_order_id, remaining_qty, &mut on_event);
                    remaining_qty.0 -= matched.0;

                    if limit.is_empty() {
                        if remove_count < prices_to_remove.len() {
                            prices_to_remove[remove_count] = ask_price;
                            remove_count += 1;
                        }
                    }
                }
                for i in 0..remove_count {
                    self.asks.remove(&prices_to_remove[i]);
                }
            }
            Side::Ask => {
                for (&bid_price, limit) in self.bids.iter_mut().rev() {
                    if bid_price < price || remaining_qty.0 == 0 {
                        break;
                    }

                    let matched = limit.fill(taker_order_id, remaining_qty, &mut on_event);
                    remaining_qty.0 -= matched.0;

                    if limit.is_empty() {
                        if remove_count < prices_to_remove.len() {
                            prices_to_remove[remove_count] = bid_price;
                            remove_count += 1;
                        }
                    }
                }
                for i in 0..remove_count {
                    self.bids.remove(&prices_to_remove[i]);
                }
            }
        }

        if remaining_qty.0 > 0 {
            on_event(MatchEvent::Maker {
                id: taker_order_id,
                price,
                qty: remaining_qty,
                side,
            });

            let target_map = match side {
                Side::Bid => &mut self.bids,
                Side::Ask => &mut self.asks,
            };

            target_map
                .entry(price)
                .or_insert_with(|| Limit::new(price))
                .orders
                .push_back(Order {
                    id: taker_order_id,
                    size: remaining_qty,
                    side,
                });
        }
    }
}