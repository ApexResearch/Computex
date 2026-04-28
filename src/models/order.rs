use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Matching,
    Matched,
    Executing,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PricingModel {
    FixedPrice,
    BidAsk,
    DynamicAuction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_type: OrderType,
    pub compute_type: String,
    pub quantity: u32,
    pub price: Decimal,
    pub currency: String,
    pub duration_hours: u32,
    pub status: OrderStatus,
    pub pricing_model: PricingModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub order_type: String,
    pub compute_type: String,
    pub quantity: u32,
    pub price: Decimal,
    pub duration_hours: u32,
    pub pricing_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMatch {
    pub id: Uuid,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub quantity: u32,
    pub price: Decimal,
    pub matched_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesContract {
    pub id: Uuid,
    pub compute_type: String,
    pub settlement_date: DateTime<Utc>,
    pub delivery_location: String,
    pub initial_price: Decimal,
    pub current_price: Decimal,
    pub contract_size: u32,
    pub open_interest: u32,
    pub volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFuturesRequest {
    pub compute_type: String,
    pub settlement_date: String,
    pub delivery_location: String,
    pub initial_price: Decimal,
    pub contract_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub compute_type: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub quantity: u32,
    pub num_orders: u32,
}

impl Order {
    pub fn new(
        user_id: Uuid,
        order_type: OrderType,
        compute_type: String,
        quantity: u32,
        price: Decimal,
        duration_hours: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            order_type,
            compute_type,
            quantity,
            price,
            currency: "USD".to_string(),
            duration_hours,
            status: OrderStatus::Pending,
            pricing_model: PricingModel::BidAsk,
            created_at: now,
            updated_at: now,
            expires_at: now + chrono::Duration::hours(24),
        }
    }
}
