use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStats {
    pub total_volume_24h: rust_decimal::Decimal,
    pub total_trades_24h: u64,
    pub avg_price_24h: rust_decimal::Decimal,
    pub highest_price_24h: rust_decimal::Decimal,
    pub lowest_price_24h: rust_decimal::Decimal,
    pub active_providers: u32,
    pub active_buyers: u32,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub compute_type: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub open: rust_decimal::Decimal,
    pub high: rust_decimal::Decimal,
    pub low: rust_decimal::Decimal,
    pub close: rust_decimal::Decimal,
    pub volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedPricing {
    pub compute_type: String,
    pub bid: rust_decimal::Decimal,
    pub ask: rust_decimal::Decimal,
    pub bid_volume: u32,
    pub ask_volume: u32,
    pub spread: rust_decimal::Decimal,
}
