# Computex API Documentation

## Base URL
```
HTTP:  http://localhost:8080
HTTPS: https://computex.example.com
API Version: v1
```

## Authentication

### JWT Token Authentication
Include JWT token in Authorization header:
```
Authorization: Bearer <jwt_token>
```

### 2FA Challenge
For operations requiring 2FA:
```
X-2FA-Challenge: <totp_token>
```

---

## Endpoints

## 1. Health & Status

### Health Check
```
GET /health
```

**Response (200 OK):**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2026-04-28T12:34:56Z"
}
```

---

## 2. User Management

### Register User
```
POST /api/v1/users/register
```

**Request:**
```json
{
  "email": "user@example.com",
  "username": "trader_001",
  "password": "SecurePassword123!",
  "account_type": "Buyer"
}
```

**Response (201 Created):**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "User registered successfully"
}
```

### Login User
```
POST /api/v1/users/login
```

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
```

**Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "requires_2fa": true
}
```

### Setup 2FA
```
POST /api/v1/users/2fa/setup
Authorization: Bearer <jwt_token>
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response (200 OK):**
```json
{
  "secret": "JBSWY3DPEBLW64TMMQ======",
  "qr_code": "data:image/png;base64,iVBORw0KGgo...",
  "backup_codes": [
    "12345678",
    "87654321",
    "..."
  ]
}
```

### Verify 2FA
```
POST /api/v1/users/2fa/verify
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "token": "123456"
}
```

**Response (200 OK):**
```json
{
  "message": "2FA verified successfully"
}
```

---

## 3. Marketplace - Orders

### Get Order Book
```
GET /api/v1/market/orderbook/{compute_type}
```

**Path Parameters:**
- `compute_type`: `inference`, `pre-training`, `rl-training`, `scaling`, `data-processing`

**Query Parameters:**
- `depth` (optional): Number of price levels to return (default: 20)

**Response (200 OK):**
```json
{
  "compute_type": "inference",
  "bids": [
    {
      "price": "10.50",
      "quantity": 100,
      "num_orders": 5
    },
    {
      "price": "10.45",
      "quantity": 250,
      "num_orders": 8
    }
  ],
  "asks": [
    {
      "price": "10.55",
      "quantity": 150,
      "num_orders": 6
    }
  ],
  "last_updated": "2026-04-28T12:34:56Z"
}
```

### Create Order
```
POST /api/v1/market/orders
Authorization: Bearer <jwt_token>
```

**Request:**
```json
{
  "order_type": "Buy",
  "compute_type": "inference",
  "quantity": 50,
  "price": "10.50",
  "duration_hours": 24,
  "pricing_model": "BidAsk"
}
```

**Response (201 Created):**
```json
{
  "order_id": "550e8400-e29b-41d4-a716-446655440001",
  "status": "pending",
  "message": "Order created successfully"
}
```

### Get Order Details
```
GET /api/v1/market/orders/{order_id}
Authorization: Bearer <jwt_token>
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "order_type": "Buy",
  "compute_type": "inference",
  "quantity": 50,
  "price": "10.50",
  "status": "pending",
  "pricing_model": "BidAsk",
  "created_at": "2026-04-28T12:00:00Z",
  "expires_at": "2026-04-29T12:00:00Z"
}
```

### Cancel Order
```
POST /api/v1/market/orders/{order_id}/cancel
Authorization: Bearer <jwt_token>
X-2FA-Challenge: <totp_token>
```

**Response (200 OK):**
```json
{
  "message": "Order cancelled successfully"
}
```

---

## 4. Providers

### List All Providers
```
GET /api/v1/providers
```

**Query Parameters:**
- `type` (optional): Filter by provider type (cloud, gpu, distributed, specialized)
- `verified_only` (optional): Only verified providers (boolean)

**Response (200 OK):**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440010",
    "name": "Vultr GPU",
    "description": "High-performance GPU compute platform",
    "provider_type": "Cloud",
    "verified": true,
    "reputation_score": 4.8,
    "total_compute_hours_provided": 5000000,
    "uptime_percentage": 99.95
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440011",
    "name": "Cerebras",
    "description": "Wafer-scale AI processors",
    "provider_type": "GPU",
    "verified": true,
    "reputation_score": 4.9,
    "total_compute_hours_provided": 1000000,
    "uptime_percentage": 99.99
  }
]
```

### Register Provider
```
POST /api/v1/providers/register
Authorization: Bearer <jwt_token>
X-2FA-Challenge: <totp_token>
```

**Request:**
```json
{
  "name": "My Compute Provider",
  "description": "GPU compute infrastructure",
  "provider_type": "Cloud",
  "api_key": "sk_live_...",
  "api_endpoint": "https://api.provider.com"
}
```

**Response (201 Created):**
```json
{
  "provider_id": "550e8400-e29b-41d4-a716-446655440020",
  "message": "Provider registered successfully"
}
```

### Get Provider Details
```
GET /api/v1/providers/{provider_id}
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "name": "Vultr GPU",
  "description": "High-performance GPU compute platform",
  "provider_type": "Cloud",
  "verified": true,
  "reputation_score": 4.8,
  "capacity": {
    "total_capacity": 10000,
    "available_capacity": 8500,
    "reserved_capacity": 1000,
    "in_use_capacity": 500
  },
  "metrics": {
    "avg_response_time_ms": 45,
    "error_rate": 0.01,
    "success_rate": 0.99,
    "total_requests": 1000000
  }
}
```

---

## 5. Futures Contracts

### List Futures
```
GET /api/v1/futures
```

**Query Parameters:**
- `compute_type` (optional): Filter by compute type
- `settlement_after` (optional): ISO 8601 date

**Response (200 OK):**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440030",
    "compute_type": "inference",
    "settlement_date": "2026-05-28T00:00:00Z",
    "delivery_location": "us-east-1",
    "initial_price": "10.00",
    "current_price": "10.25",
    "contract_size": 1000,
    "open_interest": 50000,
    "volume": 1000000
  }
]
```

### Create Futures Contract
```
POST /api/v1/futures/create
Authorization: Bearer <jwt_token>
X-2FA-Challenge: <totp_token>
```

**Request:**
```json
{
  "compute_type": "pre-training",
  "settlement_date": "2026-06-28",
  "delivery_location": "us-west-2",
  "initial_price": "15.00",
  "contract_size": 100
}
```

**Response (201 Created):**
```json
{
  "contract_id": "550e8400-e29b-41d4-a716-446655440040",
  "message": "Futures contract created"
}
```

---

## 6. Pricing & Market Data

### Get Current Prices
```
GET /api/v1/pricing/current
```

**Query Parameters:**
- `compute_types` (optional): Comma-separated list

**Response (200 OK):**
```json
[
  {
    "compute_type": "inference",
    "bid": "10.50",
    "ask": "10.55",
    "bid_volume": 1000,
    "ask_volume": 1500,
    "spread": "0.05"
  },
  {
    "compute_type": "pre-training",
    "bid": "15.00",
    "ask": "15.10",
    "bid_volume": 500,
    "ask_volume": 800,
    "spread": "0.10"
  }
]
```

### Get Price History
```
GET /api/v1/pricing/history/{compute_type}
```

**Query Parameters:**
- `period` (optional): `1h`, `4h`, `1d`, `1w`, `1m` (default: `1d`)
- `limit` (optional): Max data points (default: 100)

**Response (200 OK):**
```json
[
  {
    "compute_type": "inference",
    "timestamp": "2026-04-28T12:00:00Z",
    "open": "10.40",
    "high": "10.60",
    "low": "10.35",
    "close": "10.50",
    "volume": 50000
  },
  {
    "compute_type": "inference",
    "timestamp": "2026-04-28T13:00:00Z",
    "open": "10.50",
    "high": "10.65",
    "low": "10.48",
    "close": "10.55",
    "volume": 65000
  }
]
```

---

## Error Responses

### Standard Error Format
```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable error message",
  "code": "ERROR_CODE"
}
```

### Common Errors

**400 Bad Request:**
```json
{
  "error": "INVALID_REQUEST",
  "message": "Invalid order quantity",
  "code": "INVALID_REQUEST"
}
```

**401 Unauthorized:**
```json
{
  "error": "UNAUTHORIZED",
  "message": "Invalid JWT token",
  "code": "UNAUTHORIZED"
}
```

**404 Not Found:**
```json
{
  "error": "NOT_FOUND",
  "message": "Order not found",
  "code": "NOT_FOUND"
}
```

**500 Internal Server Error:**
```json
{
  "error": "INTERNAL_ERROR",
  "message": "An unexpected error occurred",
  "code": "INTERNAL_ERROR"
}
```

---

## Rate Limiting

- **General Limit**: 1000 requests/minute per user
- **Order Creation**: 100 orders/minute per user
- **Market Data**: Unlimited for real-time feeds

Headers:
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1234567890
```

---

## WebSocket Streaming

### Market Data Stream
```
wss://computex.example.com/ws/market/{compute_type}
```

Subscribe to real-time order book updates:
```json
{
  "type": "subscribe",
  "channels": ["orderbook", "trades", "prices"]
}
```

---

## Examples

### Python Client Example
```python
import requests
import json

BASE_URL = "http://localhost:8080"

# Register
response = requests.post(f"{BASE_URL}/api/v1/users/register", json={
    "email": "user@example.com",
    "username": "trader_001",
    "password": "SecurePassword123!",
    "account_type": "Buyer"
})
user_id = response.json()["user_id"]

# Login
response = requests.post(f"{BASE_URL}/api/v1/users/login", json={
    "email": "user@example.com",
    "password": "SecurePassword123!"
})
token = response.json()["token"]

# Create Order
headers = {"Authorization": f"Bearer {token}"}
response = requests.post(f"{BASE_URL}/api/v1/market/orders", 
    headers=headers,
    json={
        "order_type": "Buy",
        "compute_type": "inference",
        "quantity": 50,
        "price": "10.50",
        "duration_hours": 24
    }
)
print(response.json())
```

---

Generated: 2026-04-28
Version: 0.1.0
