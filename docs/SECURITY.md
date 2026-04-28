# Computex Security Documentation

## Table of Contents
1. [Authentication](#authentication)
2. [Encryption](#encryption)
3. [Post-Quantum Cryptography](#post-quantum-cryptography)
4. [Data Protection](#data-protection)
5. [Access Control](#access-control)
6. [Security Best Practices](#security-best-practices)
7. [Incident Response](#incident-response)
8. [Compliance](#compliance)

---

## Authentication

### 1. Password Security

**Storage:**
- Argon2 hashing with cryptographically secure salt
- Configuration: 
  - Memory: 19 MiB
  - Time Cost: 2 iterations
  - Parallelism: 1 thread

**Requirements:**
- Minimum 12 characters
- Must contain uppercase, lowercase, numbers, and special characters
- Not a dictionary word

**Implementation:**
```rust
pub fn hash_password(&self, password: &str) -> Result<String> {
    let salt = SaltString::generate(rand::thread_rng());
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| ComputexError::SecurityError(format!("Password hashing failed: {}", e)))
}
```

### 2. Two-Factor Authentication (2FA)

**TOTP Implementation:**
- Time-based One-Time Passwords (RFC 6238)
- 30-second time window
- 6-digit codes (1,000,000 possible combinations)
- Tolerance: ±1 time step

**Backup Codes:**
- Generated on 2FA setup: 10 codes
- 8-digit alphanumeric format
- Each code single-use
- Stored encrypted

**Flow:**
```
1. User initiates login
2. Email/password verification
3. Prompt for TOTP token
4. Verify token within ±30 second window
5. Grant access token
```

**Implementation:**
```rust
pub fn generate_totp(&self, secret: &str) -> Result<String> {
    let key = STANDARD.decode(secret)?;
    let totp = Totp::new(&key);
    Ok(format!("{:06}", totp.generate()))
}

pub fn verify_totp(&self, secret: &str, token: &str, time_window: u64) -> Result<bool> {
    // Check current time and ±1 time steps
    for time_offset in -1..=1 {
        let totp = Totp::new_with_time(&key, 30, SystemTime::now(), time_offset);
        if totp.generate() == token_num {
            return Ok(true);
        }
    }
    Ok(false)
}
```

### 3. JWT Token Management

**Token Structure:**
```json
{
  "sub": "user-id",
  "user_id": "uuid",
  "role": "buyer|seller|provider|admin",
  "exp": 1234567890,
  "iat": 1234567890,
  "nbf": 1234567890
}
```

**Security:**
- HS256 signing algorithm
- Expiration: 24 hours (configurable)
- Refresh tokens for long-session applications
- Token revocation list for logout

---

## Encryption

### 1. Data at Rest

**Primary Encryption: AES-256-GCM**

Specifications:
- Algorithm: Advanced Encryption Standard
- Key Size: 256 bits
- Mode: Galois/Counter Mode (authenticated)
- Nonce: 96-bit random value

**Use Cases:**
- Sensitive order details
- User account information
- Provider API credentials
- Price history archives

**Implementation:**
```rust
pub fn encrypt_aes_256_gcm(plaintext: &[u8], password: &str) -> Result<Vec<u8>> {
    let key = Self::derive_key(password)?;
    let cipher = Aes256Gcm::new(&key);
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher.encrypt(nonce, plaintext)?;
    
    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}
```

**Secondary Encryption: ChaCha20-Poly1305**

Specifications:
- Stream cipher: ChaCha20
- Auth tag: Poly1305
- Nonce: 96-bit random
- Key Size: 256 bits

**Use Cases:**
- High-throughput scenarios
- Real-time market data
- Streaming price updates

**Performance:** ~10x faster than AES-256-GCM

### 2. Data in Transit

**TLS/SSL:**
- Protocol: TLS 1.3 (minimum)
- Certificate: X.509 v3
- Cipher Suites:
  - TLS_AES_256_GCM_SHA384
  - TLS_CHACHA20_POLY1305_SHA256
  - TLS_AES_128_GCM_SHA256

**HSTS (HTTP Strict Transport Security):**
```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
```

**HTTPS Requirements:**
- All API endpoints must use HTTPS
- API key HTTP requests rejected
- WebSocket uses WSS only

### 3. Key Derivation

**PBKDF2 Alternative (for sensitive operations):**
```rust
fn derive_key(password: &str) -> Result<Key> {
    let salt = b"application-specific-salt";
    let iterations = 100_000;
    let key = PBKDF2::new(HmacSha256::new_from_slice(password.as_bytes()).unwrap())
        .hash_rounds(iterations, salt)
        .unwrap();
    Ok(key)
}
```

---

## Post-Quantum Cryptography

### 1. ML-KEM (Key Encapsulation Mechanism)

**Also Known As:** Kyber (NIST PQC selection)

**Replacements:**
- RSA key exchange
- Elliptic curve Diffie-Hellman

**Implementation:**
```rust
pub fn generate_ml_kem_keypair() -> Result<(Vec<u8>, Vec<u8>)> {
    // Returns (public_key, private_key)
    // Uses liboqs-rs in production
}

pub fn ml_kem_encapsulate(public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    // Returns (ciphertext, shared_secret)
}
```

**Parameters:**
- Public Key: 1184 bytes
- Private Key: 2400 bytes
- Ciphertext: 1088 bytes
- Shared Secret: 32 bytes

### 2. ML-DSA (Digital Signature Algorithm)

**Also Known As:** Dilithium (NIST PQC selection)

**Replacements:**
- RSA signatures
- ECDSA

**Implementation:**
```rust
pub fn ml_dsa_sign(private_key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    // Returns signature
}

pub fn ml_dsa_verify(public_key: &[u8], data: &[u8], signature: &[u8]) -> Result<bool> {
    // Verifies signature
}
```

**Parameters:**
- Public Key: 1312 bytes
- Private Key: 2544 bytes
- Signature: 2420 bytes

### 3. Migration Strategy

**Phase 1: Hybrid Support**
- Accept both RSA and ML-DSA signatures
- Support both TLS handshake methods
- Maintain backward compatibility

**Phase 2: Gradual Transition**
- Prefer quantum-safe in new connections
- Warn users to upgrade
- Set sunset date for RSA

**Phase 3: Full Migration**
- RSA support deprecated
- ML-DSA/ML-KEM mandatory
- Legacy systems require gateway

### 4. Threat Model

**Classical Threats:**
- Man-in-the-middle attacks: Mitigated by TLS
- Replay attacks: Prevented by nonces and timestamps
- Dictionary attacks: Mitigated by Argon2

**Quantum Threats (Post-2040 Estimated):**
- Harvest-now-decrypt-later: Mitigated by transitioning to ML-KEM/ML-DSA
- Quantum key recovery: Eliminated by post-quantum algorithms
- Shor's algorithm: No longer applicable

---

## Data Protection

### 1. Sensitive Data Classification

**Level 1: Public**
- Market prices (aggregated)
- Provider information
- General documentation

**Level 2: Internal**
- Order book (limited depth)
- Trading statistics
- System metrics

**Level 3: Confidential**
- User account details
- Individual orders
- Account balances
- Tax information

**Level 4: Highly Confidential**
- Private keys
- Backup codes
- Password hashes
- API credentials

### 2. Data Minimization

- Collect only necessary information
- Retention policies:
  - Account data: 7 years (compliance)
  - Order history: 7 years (compliance)
  - Logs: 90 days
  - Backup codes: Until used or superseded
  - Session tokens: 24 hours

### 3. HMAC for Data Integrity

```rust
pub fn create_hmac(&self, data: &str, secret: &str) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
    mac.update(data.as_bytes());
    Ok(format!("{:x}", mac.finalize().into_bytes()))
}
```

**Use Cases:**
- Order verification
- Market data integrity
- API request signing

---

## Access Control

### 1. Role-Based Access Control (RBAC)

**Roles:**
- **Buyer**: Can create buy orders, view own orders
- **Seller**: Can create sell orders, manage inventory
- **Provider**: Can register compute resources, track allocation
- **Broker**: Can facilitate trades, access market data
- **Admin**: Full system access

**Permission Matrix:**
```
                Buyer  Seller  Provider  Broker   Admin
Create Order    ✓      ✓                ✓        ✓
View Orderbook  ✓      ✓       ✓        ✓        ✓
Manage Users                                      ✓
Configure 2FA   ✓      ✓       ✓        ✓        ✓
System Config                                     ✓
```

### 2. API Key Management

**For Provider Integration:**
- API Key: 32-character random string
- API Secret: Never stored in plain text
- Rotation: Every 90 days
- Revocation: Instant

**Rate Limiting:**
- By API key tier
- Backoff strategies
- DDoS protection

---

## Security Best Practices

### 1. For Administrators

- Change default credentials immediately
- Enable 2FA on all admin accounts
- Use separate admin-only network
- Rotate API keys monthly
- Monitor access logs continuously
- Keep software updated

### 2. For Users

- Use strong, unique passwords
- Enable 2FA immediately
- Save backup codes securely
- Never share API keys
- Generate new API keys for each service
- Monitor account activity
- Report suspicious behavior

### 3. For Developers

- Never commit secrets to version control
- Use environment variables for configuration
- Validate all inputs (SQL injection, XSS prevention)
- Use parameterized queries
- Log security events
- Implement rate limiting
- Use constant-time comparisons

---

## Incident Response

### 1. Suspected Breach

**Immediate Actions:**
1. Isolate affected systems
2. Collect logs and forensic data
3. Notify affected users within 72 hours
4. Document timeline of events

**Investigation:**
- Determine cause and scope
- Identify compromised data
- Assess impact on users

**Remediation:**
- Patch vulnerabilities
- Reset affected user credentials
- Rotate all security keys
- Increase monitoring

### 2. Unauthorized Access

**Response Protocol:**
1. Revoke compromised credentials
2. Force password reset
3. Invalidate active sessions
4. Enable mandatory 2FA
5. Monitor account for activity

### 3. DDoS Attack

**Mitigation:**
- Activate rate limiting
- Geofencing if needed
- DNS blackhole sinkhole
- Contact ISP for upstream filtering

---

## Compliance

### 1. GDPR (General Data Protection Regulation)

**Requirements:**
- Right to access personal data
- Right to erasure ("right to be forgotten")
- Data portability
- Breach notification within 72 hours
- Privacy by design and default

**Implementation:**
- Encryption of personal data
- Access logs for auditing
- Data retention limits
- Consent management

### 2. CCPA (California Consumer Privacy Act)

**Requirements:**
- Consumer right to know
- Consumer right to delete
- Consumer right to opt-out
- Non-discrimination for exercising rights

### 3. HIPAA (Health Insurance Portability)

**If handling health-related compute:**
- Encryption required
- Access controls
- Audit trails
- Business Associate Agreements

### 4. PCI DSS (Payment Card Industry)

**If accepting card payments:**
- No storage of sensitive authentication data
- Encryption of data in transit
- Network segmentation
- Regular security testing

---

## Security Audit Checklist

- [ ] All passwords are hashed with Argon2
- [ ] 2FA is enabled for all administrative accounts
- [ ] TLS 1.3+ is enforced
- [ ] API keys are rotated every 90 days
- [ ] Logs are retained and monitored
- [ ] Backup codes are securely stored
- [ ] No secrets in version control
- [ ] Dependency vulnerabilities scanned weekly
- [ ] Penetration testing performed annually
- [ ] Disaster recovery plan documented

---

## Contact & Reporting

**Security Issues:**
- Email: security@computex.example.com
- PGP Key: Available via website
- Response Time: < 24 hours

---

Generated: 2026-04-28
Version: 0.1.0
