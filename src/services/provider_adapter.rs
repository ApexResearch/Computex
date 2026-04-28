use crate::error::{ComputexError, Result};
use crate::models::{ComputeProvider, ProviderCapacity, ProviderMetrics, ProviderType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct ProviderAdapterService {
    providers: Arc<RwLock<HashMap<Uuid, ComputeProvider>>>,
    capacities: Arc<RwLock<HashMap<Uuid, ProviderCapacity>>>,
    metrics: Arc<RwLock<HashMap<Uuid, ProviderMetrics>>>,
}

impl ProviderAdapterService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            capacities: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Register a new provider (Vultr, Cerebras, etc.)
    pub async fn register_provider(&self, provider: ComputeProvider) -> Result<Uuid> {
        let provider_id = provider.id;
        let mut providers = self.providers.write().await;
        providers.insert(provider_id, provider);

        // Initialize capacity
        let mut capacities = self.capacities.write().await;
        capacities.insert(
            provider_id,
            ProviderCapacity {
                provider_id,
                total_capacity: 10000,
                available_capacity: 10000,
                reserved_capacity: 0,
                in_use_capacity: 0,
                last_updated: chrono::Utc::now(),
            },
        );

        Ok(provider_id)
    }

    /// Get provider details
    pub async fn get_provider(&self, provider_id: Uuid) -> Result<ComputeProvider> {
        let providers = self.providers.read().await;
        providers
            .get(&provider_id)
            .cloned()
            .ok_or_else(|| ComputexError::NotFound(format!("Provider {} not found", provider_id)))
    }

    /// List all providers
    pub async fn list_providers(&self) -> Result<Vec<ComputeProvider>> {
        let providers = self.providers.read().await;
        Ok(providers.values().cloned().collect())
    }

    /// Get provider capacity
    pub async fn get_capacity(&self, provider_id: Uuid) -> Result<ProviderCapacity> {
        let capacities = self.capacities.read().await;
        capacities.get(&provider_id).cloned().ok_or_else(|| {
            ComputexError::NotFound(format!("Capacity for provider {} not found", provider_id))
        })
    }

    /// Reserve compute capacity
    pub async fn reserve_capacity(&self, provider_id: Uuid, amount: u32) -> Result<()> {
        let mut capacities = self.capacities.write().await;

        let capacity = capacities.get_mut(&provider_id).ok_or_else(|| {
            ComputexError::NotFound(format!("Capacity for provider {} not found", provider_id))
        })?;

        if capacity.available_capacity < amount {
            return Err(ComputexError::MarketError(
                "Insufficient capacity available".to_string(),
            ));
        }

        capacity.available_capacity -= amount;
        capacity.reserved_capacity += amount;
        capacity.last_updated = chrono::Utc::now();

        Ok(())
    }

    /// Use reserved capacity (mark as in_use)
    pub async fn use_capacity(&self, provider_id: Uuid, amount: u32) -> Result<()> {
        let mut capacities = self.capacities.write().await;

        let capacity = capacities.get_mut(&provider_id).ok_or_else(|| {
            ComputexError::NotFound(format!("Capacity for provider {} not found", provider_id))
        })?;

        if capacity.reserved_capacity < amount {
            return Err(ComputexError::MarketError(
                "Insufficient reserved capacity".to_string(),
            ));
        }

        capacity.reserved_capacity -= amount;
        capacity.in_use_capacity += amount;
        capacity.last_updated = chrono::Utc::now();

        Ok(())
    }

    /// Release capacity (mark as complete)
    pub async fn release_capacity(&self, provider_id: Uuid, amount: u32) -> Result<()> {
        let mut capacities = self.capacities.write().await;

        let capacity = capacities.get_mut(&provider_id).ok_or_else(|| {
            ComputexError::NotFound(format!("Capacity for provider {} not found", provider_id))
        })?;

        if capacity.in_use_capacity < amount {
            return Err(ComputexError::MarketError(
                "Insufficient in-use capacity".to_string(),
            ));
        }

        capacity.in_use_capacity -= amount;
        capacity.available_capacity += amount;
        capacity.last_updated = chrono::Utc::now();

        Ok(())
    }

    /// Update provider metrics
    pub async fn update_metrics(&self, metrics: ProviderMetrics) -> Result<()> {
        let mut metric_map = self.metrics.write().await;
        metric_map.insert(metrics.provider_id, metrics);
        Ok(())
    }

    /// Get provider metrics
    pub async fn get_metrics(&self, provider_id: Uuid) -> Result<ProviderMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(&provider_id).cloned().ok_or_else(|| {
            ComputexError::NotFound(format!("Metrics for provider {} not found", provider_id))
        })
    }

    /// Get provider by type
    pub async fn get_providers_by_type(&self, provider_type: ProviderType) -> Result<Vec<ComputeProvider>> {
        let providers = self.providers.read().await;
        Ok(providers
            .values()
            .filter(|p| p.provider_type == provider_type)
            .cloned()
            .collect())
    }

    /// Verify provider is healthy
    pub async fn is_provider_healthy(&self, provider_id: Uuid) -> Result<bool> {
        let metrics = self.get_metrics(provider_id).await?;
        // Provider is healthy if error rate < 5% and success rate > 95%
        Ok(metrics.error_rate < 0.05 && metrics.success_rate > 0.95)
    }

    /// Adapter for connecting to actual provider APIs
    pub async fn call_provider_api(
        &self,
        provider_id: Uuid,
        endpoint: &str,
        method: &str,
    ) -> Result<String> {
        let provider = self.get_provider(provider_id).await?;

        // This would integrate with actual provider APIs
        // For now, return a placeholder
        Ok(format!(
            "Called {} provider {} endpoint: {}",
            method, provider.name, endpoint
        ))
    }
}

impl Default for ProviderAdapterService {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to initialize ProviderAdapterService"))
    }
}
