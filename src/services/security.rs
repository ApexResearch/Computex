use crate::error::{ComputexError, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;
use totp_lite::Totp;
use uuid::Uuid;

pub struct SecurityService {
    // In-memory store for demo; use database in production
    two_factor_codes: Arc<RwLock<std::collections::HashMap<Uuid, String>>>,
}

impl SecurityService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            two_factor_codes: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| ComputexError::SecurityError(format!("Password hashing failed: {}", e)))
    }

    /// Verify a password against its hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| ComputexError::SecurityError(format!("Invalid hash format: {}", e)))?;

        let argon2 = Argon2::default();
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }

    /// Generate 2FA secret
    pub fn generate_2fa_secret(&self) -> Result<String> {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        Ok(STANDARD.encode(&random_bytes))
    }

    /// Generate TOTP-based OTP (Time-based One-Time Password)
    pub fn generate_totp(&self, secret: &str) -> Result<String> {
        let key = STANDARD
            .decode(secret)
            .map_err(|e| ComputexError::CryptoError(format!("Invalid secret: {}", e)))?;

        let totp = Totp::new(&key);
        Ok(format!("{:06}", totp.generate()))
    }

    /// Verify TOTP token (with time window tolerance)
    pub fn verify_totp(&self, secret: &str, token: &str, time_window: u64) -> Result<bool> {
        let key = STANDARD
            .decode(secret)
            .map_err(|e| ComputexError::CryptoError(format!("Invalid secret: {}", e)))?;

        let token_num: u32 = token
            .parse()
            .map_err(|_| ComputexError::InvalidRequest("Invalid token format".to_string()))?;

        // Check current time and a few time steps back/forward
        for time_offset in -1..=1 {
            let totp = Totp::new_with_time(&key, 30, std::time::SystemTime::now(), time_offset);
            if totp.generate() == token_num {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Generate backup codes for 2FA
    pub fn generate_backup_codes(&self, count: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        (0..count)
            .map(|_| {
                let random: u32 = rng.gen();
                format!("{:08}", random)
            })
            .collect()
    }

    /// Store 2FA secret for a user
    pub async fn store_2fa_secret(&self, user_id: Uuid, secret: String) -> Result<()> {
        let mut codes = self.two_factor_codes.write().await;
        codes.insert(user_id, secret);
        Ok(())
    }

    /// Retrieve 2FA secret for a user
    pub async fn get_2fa_secret(&self, user_id: Uuid) -> Result<Option<String>> {
        let codes = self.two_factor_codes.read().await;
        Ok(codes.get(&user_id).cloned())
    }

    /// Hash sensitive data for confidential storage
    pub fn hash_sensitive_data(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create an HMAC for data integrity verification
    pub fn create_hmac(&self, data: &str, secret: &str) -> Result<String> {
        use hmac::{Hmac, Mac};

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| ComputexError::SecurityError(format!("HMAC error: {}", e)))?;
        mac.update(data.as_bytes());

        Ok(format!("{:x}", mac.finalize().into_bytes()))
    }

    /// Verify an HMAC
    pub fn verify_hmac(&self, data: &str, secret: &str, expected_hmac: &str) -> Result<bool> {
        use hmac::{Hmac, Mac};

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| ComputexError::SecurityError(format!("HMAC error: {}", e)))?;
        mac.update(data.as_bytes());

        let computed = format!("{:x}", mac.finalize().into_bytes());
        Ok(computed == expected_hmac)
    }
}

impl Default for SecurityService {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to initialize SecurityService"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let service = SecurityService::new().unwrap();
        let password = "SecurePassword123!";
        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_hmac() {
        let service = SecurityService::new().unwrap();
        let data = "sensitive_data";
        let secret = "secret_key";

        let hmac = service.create_hmac(data, secret).unwrap();
        assert!(service.verify_hmac(data, secret, &hmac).unwrap());
        assert!(!service.verify_hmac("other_data", secret, &hmac).unwrap());
    }
}
