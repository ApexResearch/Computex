# Getting Started with Computex

This guide will help you set up and start using the Computex marketplace platform.

## Table of Contents
1. [Quick Start (5 minutes)](#quick-start)
2. [Development Setup](#development-setup)
3. [Production Deployment](#production-deployment)
4. [First Trade](#first-trade)
5. [Integration](#integration)
6. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Fastest Method: Docker Compose

```bash
# Clone repository
git clone https://github.com/apexresearch/computex.git
cd computex

# Start all services (PostgreSQL, Redis, Computex)
docker-compose up -d

# Wait for services to be healthy
docker-compose ps

# Check logs
docker-compose logs -f computex
```

**Server is now running at:** `http://localhost:8080`

**Access endpoints:**
- API: http://localhost:8080/api/v1
- Health: http://localhost:8080/health
- Prometheus: http://localhost:9090 (metrics)
- Grafana: http://localhost:3000 (dashboards)

---

## Development Setup

### Prerequisites
- Rust 1.70+
- PostgreSQL 13+
- Redis 6.0+
- Git

### Step 1: Install Rust

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows (download installer from https://rustup.rs/)
```

### Step 2: Clone Repository

```bash
git clone https://github.com/apexresearch/computex.git
cd computex
```

### Step 3: Setup Database

**macOS (Homebrew):**
```bash
brew install postgresql redis
brew services start postgresql
brew services start redis
```

**Ubuntu/Debian:**
```bash
sudo apt-get install postgresql redis-server
sudo systemctl start postgresql
sudo systemctl start redis-server
```

**Create database:**
```bash
sudo -u postgres createdb computex
sudo -u postgres createuser computex_user
sudo -u postgres psql -c "ALTER USER computex_user PASSWORD 'dev_password';"
```

### Step 4: Configure Environment

```bash
cp .env.example .env
```

Edit `.env`:
```bash
DATABASE_URL=postgres://computex_user:dev_password@localhost:5432/computex
REDIS_URL=redis://localhost:6379
JWT_SECRET=$(openssl rand -base64 32)
LOG_LEVEL=debug
```

### Step 5: Build & Run

```bash
# Build release binary
cargo build --release

# Run migrations (first time only)
cargo install sqlx-cli
sqlx migrate run

# Start server
cargo run --release
# or use the binary:
./target/release/computex
```

**Server output:**
```
🚀 Computex Server listening on 0.0.0.0:8080
```

### Step 6: Test the API

```bash
# Health check
curl http://localhost:8080/health

# Register user
curl -X POST http://localhost:8080/api/v1/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "trader@example.com",
    "username": "trader_001",
    "password": "SecurePassword123!",
    "account_type": "Buyer"
  }'
```

---

## Production Deployment

### Option 1: Docker (Single Server)

```bash
# Build Docker image
docker build -t computex:latest .

# Create .env file with production secrets
cat > .env.prod << EOF
COMPUTEX_ENV=production
DATABASE_URL=postgres://user:$(openssl rand -base64 32)@db.internal:5432/computex
JWT_SECRET=$(openssl rand -base64 64)
EOF

# Run with Docker
docker run -p 443:8080 \
  --env-file .env.prod \
  --name computex-prod \
  computex:latest
```

### Option 2: Kubernetes (Scalable)

```bash
# Update secrets in k8s/secrets.yaml
nano k8s/secrets.yaml

# Deploy to cluster
kubectl create namespace computex
kubectl apply -f k8s/secrets.yaml
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
kubectl apply -f k8s/ingress.yaml

# Check deployment
kubectl get deployment -n computex
kubectl logs -f deployment/computex -n computex
```

### Option 3: Manual Linux Server

```bash
# SSH into server
ssh ubuntu@your-server.com

# Install dependencies
sudo apt-get update
sudo apt-get install -y postgresql redis-server nginx certbot

# Clone and build
git clone https://github.com/apexresearch/computex.git /opt/computex
cd /opt/computex
cargo build --release

# Configure systemd service
sudo cp computex.service /etc/systemd/system/
sudo systemctl enable computex
sudo systemctl start computex

# Configure Nginx reverse proxy
sudo cp nginx.conf /etc/nginx/sites-available/computex
sudo ln -s /etc/nginx/sites-available/computex /etc/nginx/sites-enabled/
sudo systemctl restart nginx

# Setup HTTPS with Let's Encrypt
sudo certbot certonly --nginx -d computex.example.com
```

---

## First Trade

### Step 1: Create Account

```bash
curl -X POST http://localhost:8080/api/v1/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "buyer@example.com",
    "username": "buyer_001",
    "password": "BuyerPass123!",
    "account_type": "Buyer"
  }'
```

Response:
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "User registered successfully"
}
```

### Step 2: Setup 2FA (Optional but Recommended)

```bash
# Save user_id from previous response
USER_ID="550e8400-e29b-41d4-a716-446655440000"

# Generate 2FA secret
curl -X POST http://localhost:8080/api/v1/users/2fa/setup \
  -H "Content-Type: application/json" \
  -d "{ \"user_id\": \"$USER_ID\" }"
```

Response:
```json
{
  "secret": "JBSWY3DPEBLW64TMMQ======",
  "qr_code": "data:image/png;base64,iVBORw0KGgo...",
  "backup_codes": ["12345678", "87654321", ...]
}
```

### Step 3: Login

```bash
curl -X POST http://localhost:8080/api/v1/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "buyer@example.com",
    "password": "BuyerPass123!"
  }'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "requires_2fa": false
}
```

### Step 4: View Order Book

```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

curl -X GET http://localhost:8080/api/v1/market/orderbook/inference \
  -H "Authorization: Bearer $TOKEN"
```

### Step 5: Create Buy Order

```bash
curl -X POST http://localhost:8080/api/v1/market/orders \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "order_type": "Buy",
    "compute_type": "inference",
    "quantity": 100,
    "price": "10.50",
    "duration_hours": 24,
    "pricing_model": "BidAsk"
  }'
```

Response:
```json
{
  "order_id": "550e8400-e29b-41d4-a716-446655440001",
  "status": "pending",
  "message": "Order created successfully"
}
```

### Step 6: Check Order Status

```bash
ORDER_ID="550e8400-e29b-41d4-a716-446655440001"

curl -X GET http://localhost:8080/api/v1/market/orders/$ORDER_ID \
  -H "Authorization: Bearer $TOKEN"
```

---

## Integration

### Python Client

```python
import requests
import json

class ComputexClient:
    def __init__(self, base_url="http://localhost:8080"):
        self.base_url = base_url
        self.token = None

    def register(self, email, username, password, account_type):
        response = requests.post(
            f"{self.base_url}/api/v1/users/register",
            json={
                "email": email,
                "username": username,
                "password": password,
                "account_type": account_type
            }
        )
        return response.json()

    def login(self, email, password):
        response = requests.post(
            f"{self.base_url}/api/v1/users/login",
            json={"email": email, "password": password}
        )
        data = response.json()
        self.token = data["token"]
        return data

    def create_order(self, order_type, compute_type, quantity, price, duration_hours):
        response = requests.post(
            f"{self.base_url}/api/v1/market/orders",
            headers={"Authorization": f"Bearer {self.token}"},
            json={
                "order_type": order_type,
                "compute_type": compute_type,
                "quantity": quantity,
                "price": price,
                "duration_hours": duration_hours,
                "pricing_model": "BidAsk"
            }
        )
        return response.json()

    def get_orderbook(self, compute_type):
        response = requests.get(
            f"{self.base_url}/api/v1/market/orderbook/{compute_type}",
            headers={"Authorization": f"Bearer {self.token}"}
        )
        return response.json()

# Usage
client = ComputexClient()
client.register("user@example.com", "user_001", "Password123!", "Buyer")
client.login("user@example.com", "Password123!")
order = client.create_order("Buy", "inference", 50, "10.50", 24)
print(order)
```

### JavaScript/Node.js

```javascript
const axios = require('axios');

class ComputexClient {
    constructor(baseUrl = 'http://localhost:8080') {
        this.baseUrl = baseUrl;
        this.token = null;
    }

    async register(email, username, password, accountType) {
        const response = await axios.post(
            `${this.baseUrl}/api/v1/users/register`,
            { email, username, password, account_type: accountType }
        );
        return response.data;
    }

    async login(email, password) {
        const response = await axios.post(
            `${this.baseUrl}/api/v1/users/login`,
            { email, password }
        );
        this.token = response.data.token;
        return response.data;
    }

    async createOrder(orderType, computeType, quantity, price, durationHours) {
        const response = await axios.post(
            `${this.baseUrl}/api/v1/market/orders`,
            {
                order_type: orderType,
                compute_type: computeType,
                quantity,
                price,
                duration_hours: durationHours,
                pricing_model: 'BidAsk'
            },
            { headers: { Authorization: `Bearer ${this.token}` } }
        );
        return response.data;
    }

    async getOrderbook(computeType) {
        const response = await axios.get(
            `${this.baseUrl}/api/v1/market/orderbook/${computeType}`,
            { headers: { Authorization: `Bearer ${this.token}` } }
        );
        return response.data;
    }
}

// Usage
const client = new ComputexClient();
```

---

## Troubleshooting

### Common Issues

**1. Database Connection Error**
```
Error: could not connect to server: Connection refused
```
Solution: Ensure PostgreSQL is running:
```bash
sudo systemctl start postgresql  # Linux
brew services start postgresql  # macOS
```

**2. Port Already in Use**
```
Error: Address in use (os error 48)
```
Solution: Change port in .env or kill the process:
```bash
lsof -i :8080
kill -9 <PID>
```

**3. JWT Secret Too Short**
```
JWT_SECRET must be at least 32 characters
```
Solution: Generate a strong secret:
```bash
openssl rand -base64 32
```

**4. Docker Container Won't Start**
```bash
# Check logs
docker-compose logs computex

# Troubleshoot database connection
docker-compose exec computex env
```

**5. Migration Fails**
```bash
# Reset database (development only)
sqlx database drop
sqlx database create
sqlx migrate run
```

### Useful Commands

```bash
# View logs
cargo run --release 2>&1 | grep ERROR

# Test with different log level
RUST_LOG=debug cargo run

# Run in background
nohup cargo run --release > computex.log 2>&1 &

# Monitor performance
top -p $(pgrep computex)
```

---

## Next Steps

1. **Read the Documentation**
   - [Architecture](docs/ARCHITECTURE.md)
   - [API Reference](docs/API.md)
   - [Security](docs/SECURITY.md)

2. **Explore Features**
   - Browse available providers
   - Check current market prices
   - Review sample orders

3. **Join the Community**
   - GitHub Discussions
   - Discord Server
   - Developer Forum

4. **Deploy to Production**
   - Follow [Deployment Guide](docs/DEPLOYMENT.md)
   - Configure SSL/TLS
   - Set up monitoring

---

**Happy trading! 🚀**
