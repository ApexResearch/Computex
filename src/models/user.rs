use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub account_type: AccountType,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_verified: bool,
    pub is_2fa_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    Buyer,
    Seller,
    Provider,
    Broker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistrationRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub account_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub token: String,
    pub user_id: Uuid,
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup2FARequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup2FAResponse {
    pub secret: String,
    pub qr_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FARequest {
    pub user_id: Uuid,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuth {
    pub user_id: Uuid,
    pub secret: String,
    pub backup_codes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        email: String,
        username: String,
        account_type: AccountType,
        password_hash: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            username,
            account_type,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_verified: false,
            is_2fa_enabled: false,
        }
    }
}
