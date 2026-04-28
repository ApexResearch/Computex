# Quantum-Safe Cryptography for Computex

## Table of Contents
1. [Introduction](#introduction)
2. [Post-Quantum Threats](#post-quantum-threats)
3. [NIST Post-Quantum Cryptography](#nist-post-quantum-cryptography)
4. [Algorithm Details](#algorithm-details)
5. [Implementation](#implementation)
6. [Migration Strategy](#migration-strategy)
7. [Testing & Validation](#testing--validation)
8. [References](#references)

---

## Introduction

Computex implements quantum-safe cryptographic algorithms to protect against future threats posed by quantum computing. This document outlines the cryptographic strategies, algorithms, and migration path to ensure long-term security for the marketplace.

### Why Quantum Safety?

**Threat Timeline:**
- **2025-2030**: Quantum computers with ~1000-2000 qubits (insufficient for cryptanalysis)
- **2030-2040**: Quantum computers with millions of qubits (RSA-2048 breakable)
- **2040+**: Large-scale quantum computers (threat to current cryptography)

**Immediate Action Required:**
- "Harvest Now, Decrypt Later" attacks already happening
- Adversaries collecting encrypted traffic today for future decryption
- Transition must begin immediately, not at quantum breakthrough

---

## Post-Quantum Threats

### 1. Threats to Current Cryptography

**RSA (Key Exchange & Signing):**
- Key size: 2048-4096 bits
- Quantum threat: Shor's algorithm
- Breaks in: ~8 hours on 1 million qubit computer
- Current usage: TLS handshakes, API signing, order verification

**ECDSA (Digital Signatures):**
- Key size: 256-384 bits
- Quantum threat: Shor's algorithm (variant)
- Breaks in: ~1 hour on 1 million qubit computer
- Current usage: Blockchain-like order commits

**Elliptic Curve Diffie-Hellman (ECDH):**
- Key sizes: P-256, P-384
- Quantum threat: Shor's algorithm
- Breaks in: Similar to ECDSA
- Current usage: Session key derivation

### 2. Harvest Now, Decrypt Later (HNDL)

**Attack Scenario:**
1. Attacker records all encrypted traffic (order details, prices, account info)
2. Quantum computer becomes available in 2040
3. Attacker decrypts all historical traffic
4. Sensitive business information, trading patterns, user identities compromised

**Mitigation:**
- Implement quantum-safe cryptography NOW
- All sensitive data encrypted with quantum-safe algorithms
- By 2040, adversary cannot decrypt

---

## NIST Post-Quantum Cryptography

### 1. NIST PQC Standardization

In August 2022, NIST announced the first post-quantum cryptography standards:

**Selected Algorithms:**

| Category | Algorithm | Basis | Security |
|----------|-----------|-------|----------|
| Key Encapsulation | ML-KEM (Kyber) | Lattices | 128-256 bits |
| Digital Signatures | ML-DSA (Dilithium) | Lattices | 128-256 bits |
| Signatures (alt) | SLH-DSA (SPHINCS+) | Hash-based | 128-256 bits |

### 2. Why Lattice-Based Cryptography?

**Advantages:**
- ✅ Resistant to known quantum algorithms
- ✅ Relatively small key sizes
- ✅ Fast performance
- ✅ Well-studied mathematical foundation
- ✅ Confidence in security assumptions

**Example Problem (Learning With Errors):**
```
Given: Matrix A, vector b = As + e (mod q)
Find: Secret vector s
Where: s is small, e is small error
Difficulty: NP-hard even for quantum algorithms
```

---

## Algorithm Details

### 1. ML-KEM (Kyber) - Key Encapsulation

**Purpose:** Secure key agreement for symmetric key derivation

**NIST Standardization:** Approved August 2022

**Parameters:**
```
ML-KEM-512:
  - Security: Equivalent to AES-128
  - Public key: 800 bytes
  - Private key: 1632 bytes
  - Ciphertext: 768 bytes
  - Shared secret: 32 bytes

ML-KEM-768:
  - Security: Equivalent to AES-192
  - Public key: 1184 bytes
  - Private key: 2400 bytes
  - Ciphertext: 1088 bytes
  - Shared secret: 32 bytes

ML-KEM-1024:
  - Security: Equivalent to AES-256
  - Public key: 1568 bytes
  - Private key: 3168 bytes
  - Ciphertext: 1568 bytes
  - Shared secret: 32 bytes
```

**Algorithm Flow:**

```
Alice                          Bob
(wants to send)               (receiver)

Generate keypair:
  (pk_B, sk_B) ← KeyGen()   [stores sk_B]
                           [publishes pk_B]

                              Receive pk_B
Send encrypted key:
  (c, K) ← Encap(pk_B)    
  [sends c to Bob]
                              
                              Receive c
                              Decrypt:
                              K' ← Decap(c, sk_B)
[K = K' ideally, use for AES/ChaCha20]
```

**Use Case in Computex:**
```
1. Provider publishes ML-KEM public key
2. Buyer generates ephemeral session key with ML-KEM
3. Session key encrypted with provider's public key
4. Uses session key for AES-256-GCM of order details
5. Post-quantum secure exchange of order books
```

### 2. ML-DSA (Dilithium) - Digital Signature

**Purpose:** Sign orders and market data for verification

**NIST Standardization:** Approved August 2022

**Parameters:**
```
ML-DSA-44:
  - Security: Equivalent to AES-128
  - Public key: 1312 bytes
  - Private key: 2544 bytes
  - Signature: 2420 bytes

ML-DSA-65:
  - Security: Equivalent to AES-192
  - Public key: 1952 bytes
  - Private key: 4000 bytes
  - Signature: 3293 bytes

ML-DSA-87:
  - Security: Equivalent to AES-256
  - Public key: 2592 bytes
  - Private key: 5216 bytes
  - Signature: 4595 bytes
```

**Algorithm Flow:**

```
Signer (Bob)                  Verifier (Alice)
Wants to sign order_data:
  Generate keys:
    (vk, sk) ← KeyGen()    [stores sk]
                           [publishes vk]
                           
Sign order:
  σ ← Sign(order_data, sk) [send order_data + σ]
  
                            Verify:
                            OK ← Verify(order_data, σ, vk)
                            [if OK, trust order_data]
```

**Use Case in Computex:**
```
1. Seller signs order with ML-DSA private key
2. Signature attached to order
3. Buyers verify signature with seller's ML-DSA public key
4. Proves order authenticity and integrity
5. Market data signed by exchange with ML-DSA
```

### 3. Alternative: SLH-DSA (SPHINCS+)

**Characteristics:**
- Hash-based signatures
- No reliance on lattice assumptions
- Slower than ML-DSA
- Stateless (no counter management)

**Usage:** Backup digital signature if lattice theory compromised

---

## Implementation

### 1. Rust Cryptographic Libraries

**Primary: liboqs-rs** (Open Quantum Safe)
```cargo
liboqs-rs = "0.9"
```

**Library Features:**
- Key encapsulation (ML-KEM)
- Digital signatures (ML-DSA, SLH-DSA)
- Hybrid support (classical + quantum-safe)
- Well-maintained, C bindings to liboqs

### 2. Session Key Derivation

```rust
use liboqs_rs::kem;

// Generate ML-KEM keypair
let kem = kem::Kem::new(kem::AlgorithmIdentifier::ML_KEM_768)
    .expect("Failed to create KEM");

let (public_key, secret_key) = kem.keypair()
    .expect("Failed to generate keypair");

// Encapsulate: generate session key
let (ciphertext, shared_secret) = kem.encapsulate(&public_key)
    .expect("Failed to encapsulate");

// Decapsulate: recover session key
let shared_secret_recovered = kem.decapsulate(&ciphertext, &secret_key)
    .expect("Failed to decapsulate");

assert_eq!(shared_secret, shared_secret_recovered);

// Use shared_secret to derive AES key
let aes_key = KDF(shared_secret, b"order_encryption");
```

### 3. Signing Orders

```rust
use liboqs_rs::sig;

// Generate ML-DSA keypair
let sig = sig::Sig::new(sig::AlgorithmIdentifier::ML_DSA_65)
    .expect("Failed to create Sig");

let (vk, sk) = sig.keypair()
    .expect("Failed to generate keypair");

// Sign order
let order_data = b"order_details";
let signature = sig.sign(order_data, &sk)
    .expect("Failed to sign");

// Verify signature
let verified = sig.verify(order_data, &signature, &vk)
    .expect("Failed to verify");

assert!(verified);
```

### 4. Quantum-Safe TLS Handshake

```
Classical TLS 1.3          ML-KEM TLS (Future)
────────────────          ──────────────────
ClientHello                ClientHello + 
ServerHello                  ML-KEM public key
Certificate                ServerHello +
  (RSA/ECDSA sign)           ML-KEM public key
ServerKeyExchange         [ML-KEM encapsulation]
  (ECDH params)           
ClientKeyExchange         Shared secret from
  (ECDH ephemeral)          ML-KEM + classical
Finished                  CertificateVerify
  (HMAC)                    (ML-DSA signature)
                           Finished
```

---

## Migration Strategy

### Phase 1: Hybrid Support (Now - 2027)

**Goal:** Accept both classical and quantum-safe

1. **API Endpoints:**
   - Accept both RSA and ML-DSA signatures
   - Support both ECDH and ML-KEM key exchange
   - Backward compatibility maintained

2. **Order Signing:**
```rust
pub async fn sign_order(order_data: &str) -> Result<OrderSignature> {
    let classical_sig = rsa_sign(order_data)?;
    let quantum_sig = ml_dsa_sign(order_data)?;
    
    Ok(OrderSignature {
        algorithm: "hybrid",
        classical: classical_sig,
        quantum: quantum_sig,
    })
}

pub async fn verify_order(order: &Order, sig: &OrderSignature) -> Result<bool> {
    let classical_valid = rsa_verify(&order.data, &sig.classical)?;
    let quantum_valid = ml_dsa_verify(&order.data, &sig.quantum)?;
    
    Ok(classical_valid && quantum_valid)
}
```

3. **Database Changes:**
```sql
ALTER TABLE orders ADD COLUMN signature_algorithm VARCHAR(50);
ALTER TABLE orders ADD COLUMN quantum_safe_signature BYTEA;
```

### Phase 2: Quantum-Preferred (2027-2030)

**Goal:** Prefer quantum-safe, accept classical

1. **New Clients:** Default to ML-DSA and ML-KEM
2. **Legacy Support:** RSA/ECDSA still accepted
3. **Warnings:** Log usage of classical algorithms
4. **Migration Incentives:**
   - Lower fees for quantum-safe orders
   - Reliability bonuses for quantum-safe providers

### Phase 3: Full Quantum Migration (2030-2035)

**Goal:** Quantum-safe algorithms only

1. **Deprecation Period:** 2 years notice
2. **Final Support:** RSA/ECDSA rejected
3. **All new orders:** ML-DSA signing only
4. **Session keys:** ML-KEM encapsulation only

### Phase 4: Post-Quantum Era (2035+)

1. **Pure quantum-safe cryptography**
2. **Lattice-based signatures and encryption**
3. **Research into next-generation algorithms**

---

## Testing & Validation

### 1. Correctness Testing

```rust
#[test]
fn test_ml_kem_roundtrip() {
    let kem = kem::Kem::new(kem::AlgorithmIdentifier::ML_KEM_768)
        .unwrap();
    
    let (public_key, secret_key) = kem.keypair().unwrap();
    let (ciphertext, shared_secret) = kem.encapsulate(&public_key).unwrap();
    let shared_secret_recovered = kem.decapsulate(&ciphertext, &secret_key)
        .unwrap();
    
    assert_eq!(shared_secret, shared_secret_recovered);
}

#[test]
fn test_ml_dsa_signature() {
    let sig = sig::Sig::new(sig::AlgorithmIdentifier::ML_DSA_65)
        .unwrap();
    
    let (vk, sk) = sig.keypair().unwrap();
    let message = b"order_123";
    let signature = sig.sign(message, &sk).unwrap();
    
    assert!(sig.verify(message, &signature, &vk).unwrap());
    assert!(!sig.verify(b"different_order", &signature, &vk).unwrap());
}
```

### 2. Performance Benchmarking

```rust
#[bench]
fn bench_ml_kem_encapsulate(b: &mut Bencher) {
    let kem = kem::Kem::new(kem::AlgorithmIdentifier::ML_KEM_768)
        .unwrap();
    let (public_key, _) = kem.keypair().unwrap();
    
    b.iter(|| {
        kem.encapsulate(&public_key).unwrap();
    });
}

#[bench]
fn bench_ml_dsa_sign(b: &mut Bencher) {
    let sig = sig::Sig::new(sig::AlgorithmIdentifier::ML_DSA_65)
        .unwrap();
    let (_, sk) = sig.keypair().unwrap();
    let message = b"test_message";
    
    b.iter(|| {
        sig.sign(message, &sk).unwrap();
    });
}
```

**Expected Performance:**
```
ML-KEM-768 Encapsulation:   ~100-200 μs
ML-KEM-768 Decapsulation:   ~100-200 μs
ML-DSA-65 Signing:          ~200-400 μs
ML-DSA-65 Verification:     ~300-500 μs
```

### 3. Compliance Testing

**NIST STS (Statistical Test Suite):**
- RNG quality for nonces
- Entropy assessment
- Randomness validation

**Post-Quantum Cryptography Standardization Process:**
- Test vectors verification
- Known-answer tests
- Algorithm implementation suite

---

## References

### NIST Standards
- FIPS 203: ML-KEM Standard (approved August 2024)
- FIPS 204: ML-DSA Standard (approved August 2024)
- FIPS 205: SLH-DSA Standard (approved August 2024)
- NIST IR 8413: Status Report on Fourth Round

### Libraries & Resources
- liboqs: https://openquantumsafe.org/
- NIST PQC: https://csrc.nist.gov/projects/post-quantum-cryptography/
- Kyber Specification: https://pq-crystals.org/kyber/
- Dilithium Specification: https://pq-crystals.org/dilithium/

### Academic Papers
- "Kyber: A CCA2-Secure Module-Lattice-Based KEM" (IACR)
- "CRYSTALS-Dilithium: Digital Signatures from Module Lattices" (IACR)
- "Post-Quantum Cryptography: Current State and Quantum Threat" (IEEE)

### Timeline
- 2022: NIST announces PQC finalists
- 2024: FIPS standards approved
- 2025-2030: Classical algorithm deprecation begins
- 2030-2035: Full quantum-safe migration
- 2035+: Pure quantum-safe era

---

## Quantum-Safe Checklist

- [ ] ML-KEM implemented for key exchange
- [ ] ML-DSA implemented for signatures
- [ ] Hybrid mode supports both classical and quantum
- [ ] Performance benchmarks documented
- [ ] Security analysis completed
- [ ] Integration tests passing
- [ ] Migration path approved
- [ ] Staff trained on quantum cryptography
- [ ] Documentation updated
- [ ] Compliance verified

---

Generated: 2026-04-28
Version: 0.1.0
