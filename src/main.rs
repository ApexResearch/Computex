mod models;
mod services;
mod api;
mod crypto;
mod config;
mod error;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Computex Marketplace Engine");

    // Load configuration
    let config = config::Config::load()?;
    info!("Configuration loaded: {:?}", config);

    // Initialize services
    let marketplace_service = services::marketplace::MarketplaceService::new()?;
    let security_service = services::security::SecurityService::new()?;
    let pricing_service = services::pricing::PricingService::new()?;

    info!("Services initialized");

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(api::handlers::health))
        
        // User & Authentication endpoints
        .route("/api/v1/users/register", post(api::handlers::register_user))
        .route("/api/v1/users/login", post(api::handlers::login_user))
        .route("/api/v1/users/2fa/setup", post(api::handlers::setup_2fa))
        .route("/api/v1/users/2fa/verify", post(api::handlers::verify_2fa))
        
        // Marketplace endpoints
        .route("/api/v1/market/orderbook/:compute_type", get(api::handlers::get_orderbook))
        .route("/api/v1/market/orders", post(api::handlers::create_order))
        .route("/api/v1/market/orders/:order_id", get(api::handlers::get_order))
        .route("/api/v1/market/orders/:order_id/cancel", post(api::handlers::cancel_order))
        
        // Provider endpoints
        .route("/api/v1/providers", get(api::handlers::list_providers))
        .route("/api/v1/providers/register", post(api::handlers::register_provider))
        .route("/api/v1/providers/:provider_id", get(api::handlers::get_provider))
        
        // Compute futures endpoints
        .route("/api/v1/futures", get(api::handlers::list_futures))
        .route("/api/v1/futures/create", post(api::handlers::create_futures_contract))
        
        // Pricing & Market Data
        .route("/api/v1/pricing/current", get(api::handlers::get_current_prices))
        .route("/api/v1/pricing/history/:compute_type", get(api::handlers::get_price_history))
        
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB limit
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await?;

    info!("🚀 Computex Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
