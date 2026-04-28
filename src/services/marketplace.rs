use crate::error::Result;
use std::sync::Arc;

pub struct MarketplaceService {
    matching_engine: Arc<crate::services::OrderMatchingService>,
    pricing_engine: Arc<crate::services::PricingService>,
    security: Arc<crate::services::SecurityService>,
}

impl MarketplaceService {
    pub fn new() -> Result<Self> {
        let matching_engine = Arc::new(crate::services::OrderMatchingService::new()?);
        let pricing_engine = Arc::new(crate::services::PricingService::new()?);
        let security = Arc::new(crate::services::SecurityService::new()?);

        Ok(Self {
            matching_engine,
            pricing_engine,
            security,
        })
    }

    pub fn matching_engine(&self) -> Arc<crate::services::OrderMatchingService> {
        self.matching_engine.clone()
    }

    pub fn pricing_engine(&self) -> Arc<crate::services::PricingService> {
        self.pricing_engine.clone()
    }

    pub fn security(&self) -> Arc<crate::services::SecurityService> {
        self.security.clone()
    }
}

impl Default for MarketplaceService {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to initialize MarketplaceService"))
    }
}
