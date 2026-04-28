# Computex: Decentralized Compute Marketplace Architecture

## Overview

Computex is a free-market, decentralized exchange platform for computational resources in the age of AI. It enables buyers and sellers to trade compute capacity (inference, training, RL, etc.) from various providers (Vultr, Cerebras, etc.) with quantum-safe security, 2-factor authentication, and confidential data handling.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    API Layer (Axum)                         │
│  REST Endpoints for Trading & Provider Management           │
└──────────────────────┬──────────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        │              │              │
┌───────▼─────┐ ┌─────▼──────┐ ┌────▼─────────┐
│  Security   │ │Marketplace │ │   Provider   │
│  Service    │ │  Engine    │ │  Adapter     │
│ - 2FA/TOTP  │ │ - Matching │ │ - Capacity   │
│ - Encryption│ │ - Pricing  │ │ - Metrics    │
│ - Hashing   │ │ - Orders   │ │ - Integration│
└─────────────┘ └────────────┘ └──────────────┘
        │              │              │
        └──────────────┼──────────────┘
                       │
        ┌──────────────┴──────────────┐
        │                             │
┌───────▼──────────────┐  ┌──────────▼────────┐
│   Quantum-Safe       │  │  Core Models      │
│   Cryptography       │  │  - User           │
│   - ML-KEM/Kyber     │  │  - Order          │
│   - ML-DSA/Dilithium │  │  - Provider       │
│   - Session Keys     │  │  - Compute        │
│   - AES/ChaCha20     │  │  - Futures        │
└──────────────────────┘  └───────────────────┘
```

## Key Components

### 1. Security Service (`services/security.rs`)
- **Password Management**: Argon2 hashing with salt generation
- **2FA/TOTP**: Time-based one-time passwords for two-factor authentication
- **Backup Codes**: Generated for account recovery
- **HMAC**: Data integrity verification
- **Hashing**: SHA-256 for sensitive data

**Key Functions:**
```rust
- hash_password(password: &str) -> String
- verify_password(password: &str, hash: &str) -> bool
- generate_2fa_secret() -> String
- generate_totp(secret: &str) -> String
- verify_totp(secret: &str, token: &str) -> bool
- generate_backup_codes(count: usize) -> Vec<String>
- create_hmac(data: &str, secret: &str) -> String
```

### 2. Marketplace Engine (`services/marketplace.rs`)
Aggregates core trading functionality:
- Order matching and execution
- Price discovery and dynamics
- Portfolio management

**Order Matching** (`services/matching.rs`):
- FIFO price-priority matching
- Automatic order book aggregation
- Market depth calculation

**Pricing Engine** (`services/pricing.rs`):
- VWAP (Volume Weighted Average Price)
- Moving averages and technical indicators
- Anomaly detection
- Market statistics

### 3. Provider Adapter (`services/provider_adapter.rs`)
Integration layer for compute providers:
- **Supported Providers**: 
  - Cloud: AWS, GCP, Vultr
  - GPU: Cerebras, Lambda Labs
  - Distributed: Edge networks
  - Specialized: TPU providers

**Capacity Management:**
- Reserve compute capacity
- Track utilization
- Release resources
- Monitor provider health

### 4. Quantum-Safe Cryptography (`crypto/quantum_safe.rs`)
Post-quantum resistant cryptography:
- **ML-KEM** (formerly Kyber): Key encapsulation
- **ML-DSA** (formerly Dilithium): Digital signatures
- Session key generation
- Future-proof against quantum computers

### 5. Encryption Service (`crypto/encryption.rs`)
Confidential data protection:
- **AES-256-GCM**: Authenticated encryption
- **ChaCha20-Poly1305**: Fast stream cipher
- Order detail encryption
- Secure data at rest and in transit

## Data Models

### User
- Email, username, account type (Buyer/Seller/Provider/Broker)
- 2FA credentials and backup codes
- Reputation and verification status

### Compute
Types: Inference, Pre-Training, RL, Scaling, Data Processing
Arenas: GPU Inference, Distributed Training, RL Environments, Scaled Compute

### Order
- Buy/Sell with quantity and price
- Dynamic pricing models (Fixed, Bid-Ask, Auction)
- Status tracking (Pending → Matching → Executing → Completed)

### FuturesContract
- Settlement date and delivery location
- Contract size and open interest
- Price tracking for derivatives trading

### Provider
- Name, type, reputation, verified status
- Capacity tracking (total, available, reserved, in-use)
- Metrics (response time, error rate, uptime)

## API Endpoints

### Authentication
```
POST   /api/v1/users/register        Register new account
POST   /api/v1/users/login           Authenticate user
POST   /api/v1/users/2fa/setup       Initialize 2FA
POST   /api/v1/users/2fa/verify      Verify 2FA token
```

### Marketplace
```
GET    /api/v1/market/orderbook/:compute_type    Get market depth
POST   /api/v1/market/orders                      Create order
GET    /api/v1/market/orders/:order_id            Get order details
POST   /api/v1/market/orders/:order_id/cancel     Cancel order
```

### Providers
```
GET    /api/v1/providers                          List all providers
POST   /api/v1/providers/register                 Register provider
GET    /api/v1/providers/:provider_id             Provider details
```

### Futures
```
GET    /api/v1/futures                            List futures contracts
POST   /api/v1/futures/create                     Create futures contract
```

### Pricing
```
GET    /api/v1/pricing/current                    Current bid-ask spreads
GET    /api/v1/pricing/history/:compute_type      Historical prices
```

## Security Architecture

### 1. Authentication & Authorization
- JWT tokens with configurable expiry
- 2FA via TOTP (Time-based One-Time Passwords)
- Account verification workflow
- Multi-level permission model

### 2. Data Encryption
**In Transit:**
- TLS 1.3+ for all API communications
- Quantum-safe key exchange (ML-KEM ready)

**At Rest:**
- AES-256-GCM for sensitive data
- ChaCha20-Poly1305 for high-performance scenarios
- Separate encryption keys per data category

### 3. Confidentiality
- Encrypted order books (hiding individual order details)
- Privacy-preserving price aggregation
- Zero-knowledge proofs for verification (future)

### 4. Post-Quantum Readiness
- ML-KEM for key encapsulation
- ML-DSA for digital signatures
- Migration path from RSA/ECDSA
- Hybrid crypto support

## Performance Characteristics

| Operation | Latency | Throughput |
|-----------|---------|-----------|
| Order Creation | ~10ms | 1000+ orders/sec |
| Price Update | ~5ms | Real-time |
| Order Matching | ~50ms | Batch processing |
| Encryption (AES-256-GCM) | ~1ms | 1GB+/sec |
| TOTP Generation | ~0.1ms | 10k codes/sec |

## Scalability Design

### Horizontal Scaling
```
Load Balancer (nginx/HAProxy)
    ├── Computex Instance 1
    ├── Computex Instance 2
    └── Computex Instance N
         ↓
    Shared: PostgreSQL, Redis Cache
```

### Database
- PostgreSQL for transactional data (orders, users)
- TimescaleDB extension for price time-series
- Redis for order book caching

### Message Queue
- AMQP/RabbitMQ or Kafka for async operations
- Event sourcing for order lifecycle
- Market data streaming

## Deployment

### Docker
```bash
docker build -t computex:latest .
docker run -p 8080:8080 computex:latest
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: computex
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: computex
        image: computex:latest
        ports:
        - containerPort: 8080
```

### Environment Variables
```
COMPUTEX_ENV=production
COMPUTEX_HOST=0.0.0.0
COMPUTEX_PORT=8080
COMPUTEX_DB_URL=postgres://user:pass@db:5432/computex
COMPUTEX_JWT_SECRET=<strong-secret>
COMPUTEX_ENABLE_2FA=true
COMPUTEX_ENABLE_QUANTUM_CRYPTO=true
```

## Testing

### Unit Tests
```bash
cargo test --lib
```

### Integration Tests
```bash
cargo test --test '*'
```

### Load Testing
```bash
cargo install cargo-benchmark
cargo bench
```

## Monitoring & Observability

### Logging
- Structured logging with tracing
- Log levels: DEBUG, INFO, WARN, ERROR
- Correlation IDs for request tracing

### Metrics
- Prometheus metrics endpoint
- Key metrics:
  - Order processing latency
  - Matching engine throughput
  - Provider health scores
  - Security event frequency

### Alerting
- Order match failures
- Provider downtime
- Security anomalies
- Performance degradation

## Roadmap

### Phase 1 (Current)
- ✅ Core marketplace engine
- ✅ Basic security (passwords, HMAC)
- ✅ Order matching
- ✅ Provider management

### Phase 2
- Multi-asset trading
- Advanced order types (stop-loss, trailing stops)
- Portfolio analytics
- Real-time market data streaming

### Phase 3
- Full quantum-safe migration
- Decentralized consensus (smart contracts)
- Cross-chain settlements
- Compliance automation

### Phase 4
- AI-powered market making
- Automated broker strategies
- Machine learning order prediction
- Federated learning across providers

## References

- NIST Post-Quantum Cryptography: https://csrc.nist.gov/projects/post-quantum-cryptography/
- Rust Security: https://anssi-fr.github.io/rust-guide/
- Marketplace Design: https://en.wikipedia.org/wiki/Matching_engine
- Quantum Cryptography: https://en.wikipedia.org/wiki/Post-quantum_cryptography

---
Generated: 2026-04-28
Version: 0.1.0
