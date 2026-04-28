use crate::error::{ComputexError, Result};
use crate::models::{Order, OrderMatch, OrderStatus, OrderType};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct OrderMatchingService {
    // Order books by compute type
    order_books: Arc<RwLock<HashMap<String, Vec<Order>>>>,
    // Completed matches
    matches: Arc<RwLock<Vec<OrderMatch>>>,
}

impl OrderMatchingService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            order_books: Arc::new(RwLock::new(HashMap::new())),
            matches: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Add order to order book
    pub async fn add_order(&self, order: Order) -> Result<()> {
        let mut books = self.order_books.write().await;
        books
            .entry(order.compute_type.clone())
            .or_insert_with(Vec::new)
            .push(order);

        Ok(())
    }

    /// Get order book for a compute type
    pub async fn get_order_book(&self, compute_type: &str) -> Result<Vec<Order>> {
        let books = self.order_books.read().await;
        Ok(books.get(compute_type).cloned().unwrap_or_default())
    }

    /// Match orders using FIFO with price priority
    pub async fn match_orders(&self, compute_type: &str) -> Result<Vec<OrderMatch>> {
        let mut books = self.order_books.write().await;
        let mut matches = self.matches.write().await;

        let orders = books
            .get_mut(compute_type)
            .ok_or_else(|| ComputexError::NotFound("Order book not found".to_string()))?
            .clone();

        // Separate buy and sell orders
        let mut buy_orders: Vec<_> = orders
            .iter()
            .filter(|o| o.order_type == OrderType::Buy && o.status == OrderStatus::Pending)
            .cloned()
            .collect();

        let mut sell_orders: Vec<_> = orders
            .iter()
            .filter(|o| o.order_type == OrderType::Sell && o.status == OrderStatus::Pending)
            .cloned()
            .collect();

        // Sort: highest bid first, lowest ask first
        buy_orders.sort_by(|a, b| b.price.cmp(&a.price));
        sell_orders.sort_by(|a, b| a.price.cmp(&b.price));

        let mut new_matches = Vec::new();

        for buy_order in &buy_orders {
            for sell_order in &sell_orders {
                // Check if prices cross (buy price >= sell price)
                if buy_order.price >= sell_order.price {
                    let match_quantity = std::cmp::min(buy_order.quantity, sell_order.quantity);
                    let match_price = sell_order.price; // Sell order takes precedence (maker price)

                    let order_match = OrderMatch {
                        id: Uuid::new_v4(),
                        buy_order_id: buy_order.id,
                        sell_order_id: sell_order.id,
                        quantity: match_quantity,
                        price: match_price,
                        matched_at: chrono::Utc::now(),
                    };

                    new_matches.push(order_match.clone());
                    matches.push(order_match);
                }
            }
        }

        Ok(new_matches)
    }

    /// Execute a specific order match
    pub async fn execute_match(&self, match_id: Uuid) -> Result<()> {
        let mut books = self.order_books.write().await;

        // Update order statuses to Matched/Executing
        for orders in books.values_mut() {
            for order in orders.iter_mut() {
                if order.status == OrderStatus::Matching {
                    order.status = OrderStatus::Executing;
                    order.updated_at = chrono::Utc::now();
                }
            }
        }

        Ok(())
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: Uuid) -> Result<()> {
        let mut books = self.order_books.write().await;

        for orders in books.values_mut() {
            if let Some(order) = orders.iter_mut().find(|o| o.id == order_id) {
                if order.status == OrderStatus::Pending || order.status == OrderStatus::Matching {
                    order.status = OrderStatus::Cancelled;
                    order.updated_at = chrono::Utc::now();
                    return Ok(());
                }
            }
        }

        Err(ComputexError::NotFound("Order not found or cannot be cancelled".to_string()))
    }

    /// Get all completed matches
    pub async fn get_matches(&self) -> Result<Vec<OrderMatch>> {
        let matches = self.matches.read().await;
        Ok(matches.clone())
    }

    /// Get aggregated orderbook levels (market depth)
    pub async fn get_market_depth(
        &self,
        compute_type: &str,
        levels: usize,
    ) -> Result<crate::models::OrderBook> {
        let books = self.order_books.read().await;
        let orders = books.get(compute_type).ok_or_else(|| {
            ComputexError::NotFound("Order book not found".to_string())
        })?;

        // Group by price level
        let mut bid_levels: HashMap<&Decimal, (u32, u32)> = HashMap::new();
        let mut ask_levels: HashMap<&Decimal, (u32, u32)> = HashMap::new();

        for order in orders {
            if order.status == OrderStatus::Pending {
                match order.order_type {
                    OrderType::Buy => {
                        let (qty, count) = bid_levels.get(&order.price).cloned().unwrap_or((0, 0));
                        bid_levels.insert(&order.price, (qty + order.quantity, count + 1));
                    }
                    OrderType::Sell => {
                        let (qty, count) = ask_levels.get(&order.price).cloned().unwrap_or((0, 0));
                        ask_levels.insert(&order.price, (qty + order.quantity, count + 1));
                    }
                }
            }
        }

        // Convert to sorted vectors
        let mut bids: Vec<_> = bid_levels
            .iter()
            .map(|(price, (qty, count))| crate::models::OrderBookLevel {
                price: **price,
                quantity: *qty,
                num_orders: *count,
            })
            .collect();

        let mut asks: Vec<_> = ask_levels
            .iter()
            .map(|(price, (qty, count))| crate::models::OrderBookLevel {
                price: **price,
                quantity: *qty,
                num_orders: *count,
            })
            .collect();

        // Sort
        bids.sort_by(|a, b| b.price.cmp(&a.price));
        asks.sort_by(|a, b| a.price.cmp(&b.price));

        // Limit to requested levels
        bids.truncate(levels);
        asks.truncate(levels);

        Ok(crate::models::OrderBook {
            compute_type: compute_type.to_string(),
            bids,
            asks,
            last_updated: chrono::Utc::now(),
        })
    }
}

impl Default for OrderMatchingService {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to initialize OrderMatchingService"))
    }
}
