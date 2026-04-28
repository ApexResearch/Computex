use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeProvider {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub provider_type: ProviderType,
    pub verified: bool,
    pub reputation_score: f32,
    pub total_compute_hours_provided: u64,
    pub uptime_percentage: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProviderType {
    Cloud,      // Vultr, AWS, GCP, etc.
    GPU,        // Cerebras, Lambda Labs, etc.
    Distributed, // Edge computing networks
    Specialized, // TPU, custom hardware
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistrationRequest {
    pub name: String,
    pub description: String,
    pub provider_type: String,
    pub api_key: String,
    pub api_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapacity {
    pub provider_id: Uuid,
    pub total_capacity: u32,
    pub available_capacity: u32,
    pub reserved_capacity: u32,
    pub in_use_capacity: u32,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    pub provider_id: Uuid,
    pub avg_response_time_ms: u32,
    pub error_rate: f32,
    pub success_rate: f32,
    pub total_requests: u64,
    pub failed_requests: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPricing {
    pub provider_id: Uuid,
    pub compute_type: String,
    pub price_per_hour: rust_decimal::Decimal,
    pub currency: String,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

impl ComputeProvider {
    pub fn new(name: String, description: String, provider_type: ProviderType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            provider_type,
            verified: false,
            reputation_score: 5.0,
            total_compute_hours_provided: 0,
            uptime_percentage: 100.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
