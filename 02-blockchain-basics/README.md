# Phase 2: Blockchain Basics

Build a blockchain from scratch to understand fundamental concepts.

## 🎯 Learning Objectives

- Understand block structure and linking
- Implement cryptographic hashing
- Build Merkle trees for transaction verification
- Create a simple Proof of Work system
- Validate blockchain integrity

## 📚 Projects

### ✅ 01. Block Structure
**Status**: Completed
**Location**: `01-block-structure/`

Created basic Block struct with:
- Block ID
- Data field
- Previous hash reference
- Current block hash

**Run**: `cd 01-block-structure && cargo run`

---

### 📋 02. Blockchain with Validation
**Status**: Not started
**Goal**: Link blocks together and validate the chain

**What to build**:
- `Blockchain` struct to hold multiple blocks
- `add_block()` method to append new blocks
- `is_valid()` method to verify chain integrity
- Proper hash linking between blocks

**Key Concepts**:
- Vector of blocks
- Hash chain validation
- Genesis block initialization

---

### 📋 03. Hash Functions
**Status**: Not started
**Goal**: Replace "temp_hash" with real cryptographic hashing

**What to build**:
- Implement SHA256 hashing for blocks
- Hash calculation from block data
- Compare Rust hashing vs manual implementation

**Crates to use**:
```toml
[dependencies]
sha2 = "0.10"
```

**Key Concepts**:
- SHA256 algorithm
- Block header serialization
- Hex encoding

---

### 📋 04. Merkle Tree
**Status**: Not started
**Goal**: Build a Merkle tree for efficient transaction verification

**What to build**:
- `MerkleTree` struct
- Build tree from list of transactions
- Generate Merkle root
- Create and verify Merkle proofs

**Key Concepts**:
- Binary tree structure
- Recursive hashing
- Proof verification (SPV)

---

### 📋 05. Simple Proof of Work
**Status**: Not started
**Goal**: Add mining capability with difficulty adjustment

**What to build**:
- Add `nonce` field to Block
- Implement `mine_block()` with difficulty target
- Calculate block difficulty
- Basic difficulty adjustment

**Key Concepts**:
- Mining algorithm
- Difficulty target (leading zeros)
- Nonce iteration
- Computational proof

---

## 🔧 Setup

Create a new project:
```bash
cargo new <project-name>
cd <project-name>
```

Or use a workspace approach:
```bash
# From 02-blockchain-basics/
cargo new --name <project-name> <folder-name>
```

## 📖 Resources

- [But how does bitcoin actually work? (3Blue1Brown)](https://www.youtube.com/watch?v=bBC-nXj3Ng4)
- [Mastering Bitcoin - Chapter 9: The Blockchain](https://github.com/bitcoinbook/bitcoinbook)
- [Merkle Trees Explained](https://nakamoto.com/merkle-trees/)

## ✅ Completion Checklist

- [x] Understand block structure
- [ ] Build linked blockchain
- [ ] Implement real hashing
- [ ] Create Merkle tree
- [ ] Add Proof of Work mining

## 🎓 Next Phase

Once you complete these projects, move to **Phase 3: Cryptography** to learn:
- Digital signatures
- Key pair generation
- Transaction signing
- Wallet implementation
