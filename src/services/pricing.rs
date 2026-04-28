use crate::error::Result;
use crate::models::{AggregatedPricing, PriceHistory};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PricingService {
    // In-memory price history; use time-series DB in production
    price_history: Arc<RwLock<HashMap<String, Vec<PriceHistory>>>>,
    current_prices: Arc<RwLock<HashMap<String, AggregatedPricing>>>,
}

impl PricingService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            price_history: Arc::new(RwLock::new(HashMap::new())),
            current_prices: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Record price point for a compute type
    pub async fn record_price(
        &self,
        compute_type: &str,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: u64,
    ) -> Result<()> {
        let history = PriceHistory {
            compute_type: compute_type.to_string(),
            timestamp: Utc::now(),
            open,
            high,
            low,
            close,
            volume,
        };

        let mut prices = self.price_history.write().await;
        prices
            .entry(compute_type.to_string())
            .or_insert_with(Vec::new)
            .push(history);

        Ok(())
    }

    /// Get price history for a compute type
    pub async fn get_price_history(&self, compute_type: &str) -> Result<Vec<PriceHistory>> {
        let prices = self.price_history.read().await;
        Ok(prices
            .get(compute_type)
            .cloned()
            .unwrap_or_default())
    }

    /// Update aggregated pricing (bid/ask)
    pub async fn update_aggregated_pricing(
        &self,
        compute_type: &str,
        bid: Decimal,
        ask: Decimal,
        bid_volume: u32,
        ask_volume: u32,
    ) -> Result<()> {
        let spread = ask - bid;
        let pricing = AggregatedPricing {
            compute_type: compute_type.to_string(),
            bid,
            ask,
            bid_volume,
            ask_volume,
            spread,
        };

        let mut current = self.current_prices.write().await;
        current.insert(compute_type.to_string(), pricing);

        Ok(())
    }

    /// Get current aggregated pricing
    pub async fn get_current_pricing(&self, compute_type: &str) -> Result<Option<AggregatedPricing>> {
        let current = self.current_prices.read().await;
        Ok(current.get(compute_type).cloned())
    }

    /// Calculate VWAP (Volume Weighted Average Price)
    pub async fn calculate_vwap(&self, compute_type: &str, hours: i64) -> Result<Decimal> {
        let prices = self.price_history.read().await;
        let history = prices.get(compute_type).ok_or_else(|| {
            crate::error::ComputexError::NotFound(format!(
                "No price history for {}",
                compute_type
            ))
        })?;

        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        let recent: Vec<_> = history
            .iter()
            .filter(|p| p.timestamp >= cutoff)
            .collect();

        if recent.is_empty() {
            return Err(crate::error::ComputexError::NotFound(
                "No recent price data".to_string(),
            ));
        }

        let total_value: Decimal = recent
            .iter()
            .map(|p| p.close * Decimal::from(p.volume))
            .sum();
        let total_volume: u64 = recent.iter().map(|p| p.volume).sum();

        if total_volume == 0 {
            return Ok(Decimal::ZERO);
        }

        Ok(total_value / Decimal::from(total_volume))
    }

    /// Calculate moving average
    pub async fn calculate_moving_average(
        &self,
        compute_type: &str,
        periods: usize,
    ) -> Result<Decimal> {
        let prices = self.price_history.read().await;
        let history = prices.get(compute_type).ok_or_else(|| {
            crate::error::ComputexError::NotFound(format!(
                "No price history for {}",
                compute_type
            ))
        })?;

        let recent: Vec<_> = history.iter().rev().take(periods).collect();

        if recent.is_empty() {
            return Ok(Decimal::ZERO);
        }

        let sum: Decimal = recent.iter().map(|p| p.close).sum();
        Ok(sum / Decimal::from(recent.len() as u32))
    }

    /// Detect price anomalies
    pub async fn detect_price_anomaly(
        &self,
        compute_type: &str,
        current_price: Decimal,
        std_dev_threshold: f32,
    ) -> Result<bool> {
        let prices = self.price_history.read().await;
        let history = prices.get(compute_type).ok_or_else(|| {
            crate::error::ComputexError::NotFound(format!(
                "No price history for {}",
                compute_type
            ))
        })?;

        if history.len() < 10 {
            return Ok(false); // Need more data for statistical analysis
        }

        let recent: Vec<_> = history.iter().rev().take(20).map(|p| p.close).collect();
        let mean: Decimal = recent.iter().sum::<Decimal>() / Decimal::from(recent.len() as u32);
        let variance: Decimal = recent
            .iter()
            .map(|price| (price - mean).powu(2))
            .sum::<Decimal>()
            / Decimal::from(recent.len() as u32);

        let std_dev = (variance.to_f32().unwrap_or(0.0)).sqrt();
        let z_score = ((current_price - mean).to_f32().unwrap_or(0.0)).abs() / std_dev.max(0.0001);

        Ok(z_score > std_dev_threshold as f32)
    }

    /// Get market statistics
    pub async fn get_market_stats(&self, compute_type: &str, hours: i64) -> Result<crate::models::MarketStats> {
        let prices = self.price_history.read().await;
        let history = prices.get(compute_type).ok_or_else(|| {
            crate::error::ComputexError::NotFound(format!(
                "No price history for {}",
                compute_type
            ))
        })?;

        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        let recent: Vec<_> = history
            .iter()
            .filter(|p| p.timestamp >= cutoff)
            .collect();

        if recent.is_empty() {
            return Err(crate::error::ComputexError::NotFound(
                "No data for period".to_string(),
            ));
        }

        let total_volume: u64 = recent.iter().map(|p| p.volume).sum();
        let avg_price: Decimal = recent
            .iter()
            .map(|p| p.close)
            .sum::<Decimal>()
            / Decimal::from(recent.len() as u32);
        let highest = recent.iter().map(|p| p.high).max().unwrap_or_default();
        let lowest = recent.iter().map(|p| p.low).min().unwrap_or_default();

        Ok(crate::models::MarketStats {
            total_volume_24h: Decimal::from(total_volume),
            total_trades_24h: recent.len() as u64,
            avg_price_24h: avg_price,
            highest_price_24h: highest,
            lowest_price_24h: lowest,
            active_providers: 0, // Would come from provider metrics
            active_buyers: 0,     // Would come from user activity
            timestamp: Utc::now(),
        })
    }
}

impl Default for PricingService {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to initialize PricingService"))
    }
}
