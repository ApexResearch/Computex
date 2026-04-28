# Computex: Decentralized Compute Marketplace

> **Invisible hand in the age of AI** — Free-market exchange of compute resources with quantum-safe security

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-brightgreen.svg)](https://www.rust-lang.org/)

## Overview

Computex is a scalable, decentralized marketplace platform for trading computational resources in the era of large-scale AI. It enables buyers and sellers to engage in a free market for:

- **Inference Compute**: Real-time GPU/TPU inference at scale
- **Training Compute**: Pre-training on large models across multiple providers
- **Reinforcement Learning**: Distributed RL environment compute
- **Scaling Research**: Compute for exploring scaling laws and model parameters
- **Data Processing**: Large-scale data pipeline compute

Integrated with providers like **Vultr, Cerebras, Lambda Labs**, and others with:

✅ **Quantum-Safe Cryptography** (ML-KEM, ML-DSA)  
✅ **2FA/TOTP Authentication** (FIDO2-ready)  
✅ **Confidential Data Encryption** (AES-256-GCM, ChaCha20-Poly1305)  
✅ **Dynamic Pricing Engine** (Real-time VWAP, market depth)  
✅ **Order Matching System** (FIFO + Price Priority)  
✅ **Futures Markets** (Compute derivatives)  
✅ **Decentralized Information** (No single point of failure)  

---

## Quick Start

### Prerequisites
- Rust 1.70+ ([install](https://rustup.rs/))
- PostgreSQL 13+
- Redis 6.0+

### Installation

```bash
# Clone repository
git clone https://github.com/apexresearch/computex.git
cd computex

# Build release binary
cargo build --release

# Setup environment
cp .env.example .env
# Edit .env with your database credentials

# Run migrations
sqlx migrate run

# Start server
cargo run --release
```

Server will be available at `http://localhost:8080`

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    API Layer (Axum)                         │
│    REST Endpoints for Trading & Provider Integration        │
└──────────────────────┬──────────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        │              │              │
    ┌───▼──┐      ┌────▼───┐    ┌─────▼────┐
    │Security│    │Marketplace│ │ Provider │
    │Service │    │  Engine   │ │ Adapter  │
    └────────┘    └───────────┘ └──────────┘
        │              │              │
        └──────────────┼──────────────┘
                       │
        ┌──────────────┴──────────────┐
        │                             │
    ┌───▼─────────┐         ┌────────▼────────┐
    │ Quantum-Safe│         │  Core Models    │
    │  Crypto     │         │ - User/Order    │
    │- ML-KEM/DSA │         │ - Provider      │
    │- AES-GCM    │         │ - Compute specs │
    └─────────────┘         └─────────────────┘
```

### Core Components

| Component | Purpose | Technology |
|-----------|---------|-----------|
| **Security Service** | Authentication, 2FA, encryption | Argon2, TOTP, HMAC |
| **Matching Engine** | Order matching & execution | FIFO + Price Priority |
| **Pricing Engine** | Market data & price discovery | VWAP, Moving Averages |
| **Provider Adapter** | Multi-provider integration | REST APIs, async |
| **Quantum-Safe Crypto** | Post-quantum security | ML-KEM, ML-DSA |

---

## Key Features

### 1. Order Management
```
Order Types:      Buy / Sell
Compute Types:    Inference, Pre-Training, RL, Scaling, Data Processing
Pricing Models:   Fixed Price, Bid-Ask, Dynamic Auction
Order Matching:   FIFO with Price Priority
```

### 2. Real-Time Pricing
```
VWAP Calculation:     Volume-weighted average price
Market Depth:         Bid/ask levels aggregation
Anomaly Detection:    Z-score based price outliers
Market Statistics:    24h OHLCV candles
```

### 3. Security & Privacy
```
Authentication:   JWT + 2FA/TOTP
Data Encryption:  AES-256-GCM, ChaCha20-Poly1305
Confidentiality:  Order privacy, encrypted order book
Post-Quantum:     ML-KEM key exchange, ML-DSA signatures
```

### 4. Provider Integration
```
Supported Providers:  Vultr, Cerebras, AWS, GCP, Lambda Labs
Capacity Management:  Reserve, allocate, release
Health Monitoring:    Uptime %, error rate, response time
```

### 5. Futures Markets
```
Settlement Dates:     Configurable
Delivery Locations:   Multi-region
Contract Sizes:       Standardized units
Price Discovery:      Real-time mark-to-market
```

---

## API Endpoints

### User Management
```
POST   /api/v1/users/register        Register account
POST   /api/v1/users/login           Authenticate
POST   /api/v1/users/2fa/setup       Enable 2FA
POST   /api/v1/users/2fa/verify      Verify 2FA token
```

### Marketplace
```
GET    /api/v1/market/orderbook/:compute_type    Market depth
POST   /api/v1/market/orders                      Create order
GET    /api/v1/market/orders/:order_id            Order details
POST   /api/v1/market/orders/:order_id/cancel     Cancel order
```

### Providers
```
GET    /api/v1/providers                          List providers
POST   /api/v1/providers/register                 Register provider
GET    /api/v1/providers/:provider_id             Provider details
```

### Pricing
```
GET    /api/v1/pricing/current                   Current bid-ask
GET    /api/v1/pricing/history/:compute_type     Price history
```

Full API documentation: [docs/API.md](docs/API.md)

---

## Documentation

| Document | Purpose |
|----------|---------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design & components |
| [docs/API.md](docs/API.md) | REST API reference & examples |
| [docs/SECURITY.md](docs/SECURITY.md) | Security protocols & best practices |
| [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) | Deployment guides (Docker, K8s, production) |
| [docs/QUANTUM_CRYPTOGRAPHY.md](docs/QUANTUM_CRYPTOGRAPHY.md) | Post-quantum crypto strategy |

---

## Development

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# With logging
RUST_LOG=debug cargo test

# Benchmarks
cargo bench
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Check dependencies
cargo audit

# Generate docs
cargo doc --open
```

### Project Structure
```
computex/
├── src/
│   ├── main.rs                 # Entry point
│   ├── models/                 # Data models
│   │   ├── user.rs            # User/auth models
│   │   ├── order.rs           # Order specifications
│   │   ├── compute.rs         # Compute resources
│   │   ├── provider.rs        # Provider models
│   │   └── marketplace.rs     # Market models
│   ├── services/              # Business logic
│   │   ├── security.rs        # 2FA, encryption
│   │   ├── pricing.rs         # Price engine
│   │   ├── matching.rs        # Order matching
│   │   ├── marketplace.rs     # Main service
│   │   └── provider_adapter.rs # Provider integration
│   ├── crypto/                # Cryptography
│   │   ├── quantum_safe.rs    # ML-KEM, ML-DSA
│   │   └── encryption.rs      # AES-GCM, ChaCha20
│   ├── api/                   # REST API
│   │   ├── handlers.rs        # Endpoint handlers
│   │   └── routes.rs          # Route definitions
│   ├── error.rs               # Error types
│   └── config.rs              # Configuration
├── docs/                      # Documentation
├── Cargo.toml                 # Dependencies
├── .env.example               # Environment template
└── README.md                  # This file
```

---

## Deployment

### Docker
```bash
docker build -t computex:latest .
docker run -p 8080:8080 computex:latest
```

### Docker Compose
```bash
docker-compose up -d
```

### Kubernetes
```bash
kubectl apply -f k8s/
```

Full deployment guide: [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)

---

## Roadmap

### Phase 1: Core Platform ✓
- [x] Marketplace engine
- [x] Order matching
- [x] Security & 2FA
- [x] Provider integration
- [x] Quantum-safe crypto foundation

### Phase 2: Advanced Features (2026)
- [ ] Multi-asset trading
- [ ] Advanced order types (stop-loss, bracket)
- [ ] Portfolio analytics
- [ ] Real-time WebSocket streams
- [ ] Mobile app

### Phase 3: Ecosystem (2027)
- [ ] Decentralized consensus
- [ ] Smart contract integration
- [ ] Cross-chain settlement
- [ ] Automated compliance

### Phase 4: Intelligence (2028)
- [ ] AI market making
- [ ] Machine learning prediction
- [ ] Federated learning across providers
- [ ] Autonomous traders

---

## Performance

| Operation | Latency | Throughput |
|-----------|---------|-----------|
| Order creation | ~10ms | 1000+ orders/sec |
| Price update | ~5ms | Real-time |
| Order matching | ~50ms | Batch |
| Encryption (AES-256-GCM) | ~1ms | 1GB+/sec |
| 2FA generation | ~0.1ms | 10k/sec |

---

## Security Considerations

- ✅ Passwords hashed with Argon2 + salt
- ✅ 2FA via TOTP (RFC 6238)
- ✅ JWT tokens with 24-hour expiry
- ✅ AES-256-GCM for sensitive data
- ✅ ML-KEM key encapsulation (quantum-safe)
- ✅ ML-DSA digital signatures (quantum-safe)
- ✅ TLS 1.3+ for all communications
- ✅ HMAC for message authentication

See [docs/SECURITY.md](docs/SECURITY.md) for comprehensive security documentation.

---

## Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push branch: `git push origin feature/amazing-feature`
5. Submit pull request

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

---

## Contact & Support

- **Issue Tracker**: GitHub Issues
- **Security**: security@computex.example.com
- **Documentation**: https://docs.computex.example.com

---

## Acknowledgments

- NIST Post-Quantum Cryptography Project
- Open Quantum Safe (liboqs)
- Rust Security WG
- Axum web framework team

---

**Built with ❤️ by Apex Research**  
*Bringing free markets to AI compute in the post-quantum era*
