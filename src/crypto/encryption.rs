use crate::error::{ComputexError, Result};
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305, Key};
use rand::Rng;

pub struct EncryptionService;

impl EncryptionService {
    /// Encrypt sensitive data with AES-256-GCM
    pub fn encrypt_aes_256_gcm(plaintext: &[u8], password: &str) -> Result<Vec<u8>> {
        // Derive key from password using Argon2 (in production)
        let key = Self::derive_key(password)?;
        let cipher = Aes256Gcm::new(&key);

        // Generate random nonce
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| ComputexError::CryptoError(format!("AES encryption failed: {}", e)))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt AES-256-GCM encrypted data
    pub fn decrypt_aes_256_gcm(ciphertext: &[u8], password: &str) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(ComputexError::CryptoError(
                "Invalid ciphertext length".to_string(),
            ));
        }

        let key = Self::derive_key(password)?;
        let cipher = Aes256Gcm::new(&key);

        // Extract nonce
        let (nonce_bytes, encrypted) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, encrypted)
            .map_err(|e| ComputexError::CryptoError(format!("AES decryption failed: {}", e)))
    }

    /// Encrypt with ChaCha20-Poly1305 (faster alternative)
    pub fn encrypt_chacha20(plaintext: &[u8], password: &str) -> Result<Vec<u8>> {
        let key = Self::derive_key(password)?;
        let cipher = ChaCha20Poly1305::new(&key);

        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| ComputexError::CryptoError(format!("ChaCha20 encryption failed: {}", e)))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt ChaCha20-Poly1305 encrypted data
    pub fn decrypt_chacha20(ciphertext: &[u8], password: &str) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(ComputexError::CryptoError(
                "Invalid ciphertext length".to_string(),
            ));
        }

        let key = Self::derive_key(password)?;
        let cipher = ChaCha20Poly1305::new(&key);

        let (nonce_bytes, encrypted) = ciphertext.split_at(12);
        let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, encrypted)
            .map_err(|e| ComputexError::CryptoError(format!("ChaCha20 decryption failed: {}", e)))
    }

    /// Derive a cryptographic key from password
    fn derive_key(password: &str) -> Result<aes_gcm::Key<Aes256Gcm>> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let key_bytes = hasher.finalize();

        Ok(aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes[..]).clone())
    }

    /// Encrypt order details for privacy
    pub fn encrypt_order_details(order_id: &str, user_id: &str, price: &str) -> Result<String> {
        let data = format!("{}:{}:{}", order_id, user_id, price);
        let encrypted = Self::encrypt_aes_256_gcm(data.as_bytes(), "order-encryption-key")?;

        Ok(base64::engine::general_purpose::STANDARD.encode(&encrypted))
    }

    /// Decrypt order details
    pub fn decrypt_order_details(encrypted: &str) -> Result<(String, String, String)> {
        let encrypted_bytes =
            base64::engine::general_purpose::STANDARD
                .decode(encrypted)
                .map_err(|e| ComputexError::CryptoError(format!("Base64 decode failed: {}", e)))?;

        let decrypted = Self::decrypt_aes_256_gcm(&encrypted_bytes, "order-encryption-key")?;
        let data = String::from_utf8(decrypted)
            .map_err(|e| ComputexError::CryptoError(format!("UTF-8 decode failed: {}", e)))?;

        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 3 {
            return Err(ComputexError::CryptoError("Invalid encrypted data format".to_string()));
        }

        Ok((
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_encryption() {
        let plaintext = b"Sensitive compute order data";
        let password = "secure_password";

        let ciphertext = EncryptionService::encrypt_aes_256_gcm(plaintext, password).unwrap();
        let decrypted = EncryptionService::decrypt_aes_256_gcm(&ciphertext, password).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_chacha20_encryption() {
        let plaintext = b"Sensitive compute order data";
        let password = "secure_password";

        let ciphertext = EncryptionService::encrypt_chacha20(plaintext, password).unwrap();
        let decrypted = EncryptionService::decrypt_chacha20(&ciphertext, password).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_order_encryption() {
        let encrypted =
            EncryptionService::encrypt_order_details("order-123", "user-456", "100.50").unwrap();
        let (order_id, user_id, price) =
            EncryptionService::decrypt_order_details(&encrypted).unwrap();

        assert_eq!(order_id, "order-123");
        assert_eq!(user_id, "user-456");
        assert_eq!(price, "100.50");
    }
}
