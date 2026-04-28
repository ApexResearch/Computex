/// API Routes configuration
/// All routes are defined in main.rs and assembled into the Router

pub struct ApiRoutes;

impl ApiRoutes {
    /// Health & Status
    pub const HEALTH: &'static str = "/health";

    /// User Management
    pub const REGISTER: &'static str = "/api/v1/users/register";
    pub const LOGIN: &'static str = "/api/v1/users/login";
    pub const SETUP_2FA: &'static str = "/api/v1/users/2fa/setup";
    pub const VERIFY_2FA: &'static str = "/api/v1/users/2fa/verify";

    /// Marketplace
    pub const ORDERBOOK: &'static str = "/api/v1/market/orderbook/:compute_type";
    pub const CREATE_ORDER: &'static str = "/api/v1/market/orders";
    pub const GET_ORDER: &'static str = "/api/v1/market/orders/:order_id";
    pub const CANCEL_ORDER: &'static str = "/api/v1/market/orders/:order_id/cancel";

    /// Providers
    pub const LIST_PROVIDERS: &'static str = "/api/v1/providers";
    pub const REGISTER_PROVIDER: &'static str = "/api/v1/providers/register";
    pub const GET_PROVIDER: &'static str = "/api/v1/providers/:provider_id";

    /// Futures
    pub const LIST_FUTURES: &'static str = "/api/v1/futures";
    pub const CREATE_FUTURES: &'static str = "/api/v1/futures/create";

    /// Pricing
    pub const CURRENT_PRICES: &'static str = "/api/v1/pricing/current";
    pub const PRICE_HISTORY: &'static str = "/api/v1/pricing/history/:compute_type";
}
