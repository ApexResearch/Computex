# Computex Deployment Guide

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Local Development](#local-development)
3. [Docker Deployment](#docker-deployment)
4. [Kubernetes Deployment](#kubernetes-deployment)
5. [Production Setup](#production-setup)
6. [Database Setup](#database-setup)
7. [Monitoring & Logging](#monitoring--logging)
8. [Backup & Recovery](#backup--recovery)

---

## Prerequisites

### System Requirements
- CPU: Minimum 2 cores (8+ cores recommended)
- RAM: Minimum 2GB (8GB+ recommended)
- Storage: Minimum 10GB (100GB+ recommended)
- OS: Linux (Ubuntu 20.04+, CentOS 8+) or macOS 10.15+

### Software Requirements
- Rust 1.70+ (install from https://rustup.rs/)
- PostgreSQL 13+ (for database)
- Redis 6.0+ (for caching)
- Docker & Docker Compose (optional, for containerization)
- kubectl (for Kubernetes deployments)

---

## Local Development

### 1. Installation

```bash
# Clone repository
git clone https://github.com/apexresearch/computex.git
cd computex

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Setup Database

```bash
# Install PostgreSQL
# Ubuntu:
sudo apt-get install postgresql postgresql-contrib

# macOS with Homebrew:
brew install postgresql

# Start PostgreSQL
sudo systemctl start postgresql  # Linux
brew services start postgresql   # macOS

# Create database
createdb computex
createuser computex_user -P  # Set password when prompted

# Configure .env file
cat > .env << EOF
DATABASE_URL=postgres://computex_user:password@localhost:5432/computex
REDIS_URL=redis://localhost:6379
JWT_SECRET=$(openssl rand -base64 32)
COMPUTEX_ENV=development
EOF
```

### 3. Build & Run

```bash
# Build project
cargo build --release

# Run migrations (if using sqlx-cli)
cargo install sqlx-cli
sqlx migrate run

# Run server
cargo run --release

# Server will start on http://localhost:8080
```

### 4. Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_password_hashing

# Run benchmarks
cargo bench
```

### 5. Development Tools

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open

# Check for security vulnerabilities
cargo audit

# Update dependencies
cargo update
```

---

## Docker Deployment

### 1. Dockerfile

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/computex /usr/local/bin/

EXPOSE 8080

ENV COMPUTEX_ENV=production
CMD ["computex"]
```

### 2. Docker Compose

```yaml
version: '3.8'

services:
  computex:
    build: .
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgres://user:password@postgres:5432/computex
      REDIS_URL: redis://redis:6379
      JWT_SECRET: ${JWT_SECRET}
      COMPUTEX_ENV: production
    depends_on:
      - postgres
      - redis
    volumes:
      - ./logs:/app/logs

  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: computex
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

### 3. Build & Run with Docker

```bash
# Build image
docker build -t computex:latest .

# Run container
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@db:5432/computex \
  -e JWT_SECRET=$(openssl rand -base64 32) \
  computex:latest

# Using Docker Compose
docker-compose up -d

# Check logs
docker-compose logs -f computex

# Stop services
docker-compose down
```

---

## Kubernetes Deployment

### 1. Namespace & Secrets

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: computex

---
apiVersion: v1
kind: Secret
metadata:
  name: computex-secrets
  namespace: computex
type: Opaque
stringData:
  DATABASE_URL: postgres://user:password@postgres:5432/computex
  REDIS_URL: redis://redis:6379
  JWT_SECRET: $(openssl rand -base64 32)
```

### 2. Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: computex
  namespace: computex
spec:
  replicas: 3
  selector:
    matchLabels:
      app: computex
  template:
    metadata:
      labels:
        app: computex
    spec:
      containers:
      - name: computex
        image: computex:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
          name: http
        envFrom:
        - secretRef:
            name: computex-secrets
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

### 3. Service & Ingress

```yaml
apiVersion: v1
kind: Service
metadata:
  name: computex-service
  namespace: computex
spec:
  type: LoadBalancer
  ports:
  - port: 80
    targetPort: 8080
    protocol: TCP
  selector:
    app: computex

---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: computex-ingress
  namespace: computex
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - computex.example.com
    secretName: computex-tls
  rules:
  - host: computex.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: computex-service
            port:
              number: 80
```

### 4. Deploy to Kubernetes

```bash
# Create namespace and secrets
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/secrets.yaml

# Deploy application
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
kubectl apply -f k8s/ingress.yaml

# Check deployment status
kubectl get deployments -n computex
kubectl get pods -n computex
kubectl logs -f deployment/computex -n computex

# Scale deployment
kubectl scale deployment computex --replicas=5 -n computex
```

---

## Production Setup

### 1. Environment Configuration

```bash
# Production .env
COMPUTEX_ENV=production
COMPUTEX_HOST=0.0.0.0
COMPUTEX_PORT=8080

# Database
DATABASE_URL=postgres://prod_user:complex_password@db.internal:5432/computex
DB_MAX_POOL_SIZE=50
DB_MIN_IDLE=10

# Security
JWT_SECRET=$(openssl rand -base64 64)  # Strong secret
ENABLE_2FA=true
ENABLE_QUANTUM_CRYPTO=true
TLS_CERT_PATH=/etc/computex/certs/server.crt
TLS_KEY_PATH=/etc/computex/certs/server.key

# Redis
REDIS_URL=redis://redis-cluster:6379
REDIS_PASSWORD=secure_redis_password

# Logging
LOG_LEVEL=info
LOG_FORMAT=json
LOG_OUTPUT=/var/log/computex/application.log

# Monitoring
METRICS_PORT=9090
PROMETHEUS_ENABLED=true
```

### 2. SSL/TLS Certificate

```bash
# Generate self-signed certificate (test only)
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes

# Using Let's Encrypt (production)
certbot certonly --standalone -d computex.example.com

# Copy certificates
sudo cp server.crt /etc/computex/certs/
sudo cp server.key /etc/computex/certs/
sudo chown computex:computex /etc/computex/certs/server.key
sudo chmod 600 /etc/computex/certs/server.key
```

### 3. Systemd Service

```ini
# /etc/systemd/system/computex.service
[Unit]
Description=Computex Marketplace Engine
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=computex
WorkingDirectory=/opt/computex
EnvironmentFile=/opt/computex/.env
ExecStart=/opt/computex/computex
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl enable computex
sudo systemctl start computex
sudo systemctl status computex
```

### 4. Nginx Reverse Proxy

```nginx
upstream computex {
    server 127.0.0.1:8080;
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
}

server {
    listen 80;
    server_name computex.example.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name computex.example.com;

    ssl_certificate /etc/letsencrypt/live/computex.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/computex.example.com/privkey.pem;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;

    location / {
        proxy_pass http://computex;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

---

## Database Setup

### 1. PostgreSQL Configuration

```sql
-- Create user with limited privileges
CREATE USER computex_app WITH PASSWORD 'secure_password';
CREATE DATABASE computex OWNER computex_app;

-- Create extensions
\c computex
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Create schema
CREATE SCHEMA IF NOT EXISTS marketplace AUTHORIZATION computex_app;

-- Grant privileges
GRANT CONNECT ON DATABASE computex TO computex_app;
GRANT USAGE ON SCHEMA marketplace TO computex_app;
GRANT CREATE ON SCHEMA marketplace TO computex_app;
```

### 2. Schema Initialization

Create `migrations/001_init_schema.sql`:

```sql
-- Users table
CREATE TABLE marketplace.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    account_type VARCHAR(50) NOT NULL,
    2fa_enabled BOOLEAN DEFAULT FALSE,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Orders table
CREATE TABLE marketplace.orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES marketplace.users(id),
    order_type VARCHAR(10) NOT NULL,
    compute_type VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL,
    price DECIMAL(18,8) NOT NULL,
    status VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

-- Providers table
CREATE TABLE marketplace.providers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    provider_type VARCHAR(50) NOT NULL,
    verified BOOLEAN DEFAULT FALSE,
    reputation_score FLOAT DEFAULT 5.0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_orders_user_id ON marketplace.orders(user_id);
CREATE INDEX idx_orders_status ON marketplace.orders(status);
CREATE INDEX idx_orders_compute_type ON marketplace.orders(compute_type);
CREATE INDEX idx_orders_created_at ON marketplace.orders(created_at);
```

### 3. Run Migrations

```bash
# Install sqlx-cli
cargo install sqlx-cli

# Create migrations directory
mkdir -p migrations

# Run migrations
sqlx migrate run
```

---

## Monitoring & Logging

### 1. Prometheus Metrics

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'computex'
    static_configs:
      - targets: ['localhost:9090']
```

### 2. Structured Logging

```rust
// In main.rs
tracing_subscriber::fmt()
    .json()
    .with_target(true)
    .with_thread_ids(true)
    .with_line_number(true)
    .init();
```

### 3. ELK Stack Integration

```yaml
# docker-compose addition
elasticsearch:
  image: docker.elastic.co/elasticsearch/elasticsearch:8.5.0
  environment:
    - discovery.type=single-node

logstash:
  image: docker.elastic.co/logstash/logstash:8.5.0
  volumes:
    - ./logstash.conf:/usr/share/logstash/pipeline/logstash.conf

kibana:
  image: docker.elastic.co/kibana/kibana:8.5.0
  ports:
    - "5601:5601"
```

---

## Backup & Recovery

### 1. PostgreSQL Backup

```bash
# Backup database
pg_dump -U computex_app computex > backup_$(date +%Y%m%d).sql

# Automated daily backup
0 2 * * * pg_dump -U computex_app computex > /backups/computex_$(date +\%Y\%m\%d).sql

# Restore from backup
psql -U computex_app computex < backup_20260428.sql
```

### 2. Redis Backup

```bash
# Enable persistence in redis.conf
appendonly yes
appendfsync everysec

# Create backup
redis-cli BGSAVE

# Restore backup
redis-server --dbfilename dump.rdb
```

### 3. Disaster Recovery Plan

- RTO (Recovery Time Objective): 1 hour
- RPO (Recovery Point Objective): 15 minutes
- Backup frequency: Every 6 hours
- Test recovery: Monthly
- Offsite backups: AWS S3, daily

---

Generated: 2026-04-28
Version: 0.1.0
