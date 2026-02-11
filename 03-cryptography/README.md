# Phase 3: Cryptography

Implement cryptographic primitives essential for blockchain security.

## 🎯 Learning Objectives

- Understand cryptographic hash functions
- Implement digital signatures (ECDSA, EdDSA)
- Generate and manage key pairs
- Create wallet functionality
- Sign and verify transactions

## 📚 Projects

### 01. Hash Functions
**Goal**: Implement and understand various hashing algorithms

**What to build**:
```rust
// SHA256, Keccak256, RIPEMD160
fn sha256_hash(data: &[u8]) -> Vec<u8>;
fn keccak256_hash(data: &[u8]) -> Vec<u8>;

// Hash chain (Bitcoin address derivation)
fn hash160(data: &[u8]) -> Vec<u8> {
    ripemd160(sha256(data))
}
```

**Crates**:
```toml
[dependencies]
sha2 = "0.10"
sha3 = "0.10"
ripemd = "0.1"
hex = "0.4"
```

---

### 02. Key Pair Generation
**Goal**: Generate public/private key pairs

**What to build**:
```rust
struct KeyPair {
    private_key: SecretKey,
    public_key: PublicKey
}

impl KeyPair {
    fn generate() -> Self;
    fn from_private_key(sk: &[u8]) -> Result<Self, Error>;
    fn public_key_bytes(&self) -> Vec<u8>;
}
```

**Algorithms**:
- secp256k1 (Bitcoin, Ethereum)
- ed25519 (Solana, Polkadot)

**Crates**:
```toml
secp256k1 = { version = "0.28", features = ["rand"] }
ed25519-dalek = "2.0"
rand = "0.8"
```

---

### 03. Digital Signatures
**Goal**: Sign and verify messages

**What to build**:
```rust
fn sign_message(message: &[u8], private_key: &SecretKey) -> Signature;
fn verify_signature(
    message: &[u8],
    signature: &Signature,
    public_key: &PublicKey
) -> bool;

// Ethereum-style signing
fn sign_eth_message(message: &str, private_key: &SecretKey) -> String;
```

**Concepts**:
- ECDSA signatures
- Message hashing before signing
- Signature format (r, s, v)
- Recovery ID

---

### 04. Address Generation
**Goal**: Generate blockchain addresses from public keys

**What to build**:
```rust
// Bitcoin address
fn generate_bitcoin_address(public_key: &PublicKey, network: Network) -> String;

// Ethereum address
fn generate_ethereum_address(public_key: &PublicKey) -> String {
    let hash = keccak256(public_key.serialize_uncompressed());
    format!("0x{}", hex::encode(&hash[12..]))
}

// Base58Check encoding (Bitcoin)
fn base58check_encode(payload: &[u8], version: u8) -> String;
```

---

### 05. Simple Wallet
**Goal**: Create a basic cryptocurrency wallet

**What to build**:
```rust
struct Wallet {
    key_pair: KeyPair,
    address: String
}

impl Wallet {
    fn new() -> Self;
    fn from_private_key(key: &str) -> Result<Self, Error>;
    fn sign_transaction(&self, tx: &Transaction) -> Signature;
    fn export_private_key(&self) -> String;
    fn export_mnemonic(&self) -> String; // BIP39
}
```

**Features**:
- Key generation
- Transaction signing
- Import/export keys
- Mnemonic phrases (BIP39)

**Crates**:
```toml
bip39 = "2.0"
```

---

### 06. HD Wallet (BIP32/BIP44)
**Goal**: Hierarchical Deterministic wallet

**What to build**:
```rust
struct HDWallet {
    master_key: ExtendedPrivKey,
    accounts: Vec<Account>
}

// Derivation path: m/44'/60'/0'/0/0
impl HDWallet {
    fn from_mnemonic(mnemonic: &str) -> Self;
    fn derive_account(&self, index: u32) -> Account;
    fn derive_address(&self, account: u32, index: u32) -> String;
}
```

**Concepts**:
- BIP32: Hierarchical Deterministic Wallets
- BIP39: Mnemonic phrases
- BIP44: Multi-account hierarchy

---

## 🔐 Cryptographic Primitives

### Hash Functions

| Algorithm | Use Case | Output Size |
|-----------|----------|-------------|
| SHA256 | Bitcoin mining, general hashing | 32 bytes |
| Keccak256 | Ethereum, Solidity | 32 bytes |
| RIPEMD160 | Bitcoin addresses | 20 bytes |
| BLAKE2 | Fast hashing (Zcash) | Variable |

### Signature Schemes

| Scheme | Blockchain | Key Size | Signature Size |
|--------|-----------|----------|----------------|
| ECDSA secp256k1 | Bitcoin, Ethereum | 32 bytes | 64 bytes + recovery |
| EdDSA ed25519 | Solana, Polkadot | 32 bytes | 64 bytes |
| Schnorr | Bitcoin (Taproot) | 32 bytes | 64 bytes |

## 🔧 Setup Example

```bash
cd 03-cryptography
cargo new wallet
cd wallet

# Add to Cargo.toml:
[dependencies]
secp256k1 = { version = "0.28", features = ["rand"] }
sha2 = "0.10"
sha3 = "0.10"
hex = "0.4"
rand = "0.8"
```

## 📖 Resources

**Must Read**:
- [Mastering Bitcoin - Chapter 4: Keys & Addresses](https://github.com/bitcoinbook/bitcoinbook)
- [BIP32: HD Wallets](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP39: Mnemonic Phrases](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [BIP44: Multi-Account Hierarchy](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)

**Interactive**:
- [Bitcoin Address Explorer](https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses)
- [Ethereum Address Checksum](https://eips.ethereum.org/EIPS/eip-55)

## 💡 Key Concepts

### Private Key → Public Key → Address

```
Private Key (32 bytes random)
    ↓ (ECDSA/EdDSA)
Public Key (33/65 bytes)
    ↓ (Hash + Encode)
Address (20-40 chars)
```

### Signing Process

```
1. Hash the message: hash = SHA256(message)
2. Sign the hash: signature = ECDSA_sign(hash, private_key)
3. Return signature: (r, s, v)
```

### Verification Process

```
1. Hash the message: hash = SHA256(message)
2. Verify: ECDSA_verify(hash, signature, public_key)
3. Return true/false
```

## ✅ Completion Checklist

- [ ] Implement SHA256, Keccak256 hashing
- [ ] Generate secp256k1 key pairs
- [ ] Sign and verify messages with ECDSA
- [ ] Generate Bitcoin addresses
- [ ] Generate Ethereum addresses
- [ ] Create a basic wallet
- [ ] Implement BIP39 mnemonic generation
- [ ] Build HD wallet with BIP32/44

## 🚨 Security Best Practices

1. **Never hardcode private keys** - Use environment variables
2. **Use secure random number generator** - `rand::thread_rng()`
3. **Clear sensitive data** - Zero out memory when done
4. **Validate inputs** - Check key/signature formats
5. **Use established libraries** - Don't roll your own crypto

```rust
// Good: Use secure RNG
use rand::rngs::OsRng;
let secret_key = SecretKey::new(&mut OsRng);

// Bad: Predictable keys
let secret_key = SecretKey::from_slice(&[1u8; 32])?;
```

## 🎓 Next Phase

Once you master these concepts, move to **Phase 4: Consensus Mechanisms** to learn:
- Proof of Work mining
- Proof of Stake validators
- Byzantine Fault Tolerance

---

**Time Investment**: 2-3 weeks
**Difficulty**: ⭐⭐⭐⭐ (Crypto is complex but fascinating!)
**Prerequisites**: Strong understanding of Phase 1 (Rust basics)
