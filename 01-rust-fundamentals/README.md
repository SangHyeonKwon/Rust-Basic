# Phase 1: Rust Fundamentals

Master core Rust concepts essential for blockchain development.

## 🎯 Learning Objectives

- Understand ownership, borrowing, and lifetimes
- Master structs, enums, and pattern matching
- Learn traits and generics for code reuse
- Handle errors properly with Result and Option
- Work with collections (Vec, HashMap, BTreeMap)

## 📚 Mini Projects

### 01. Key-Value Store
**Goal**: Learn HashMap and ownership patterns

**What to build**:
```rust
struct KeyValueStore {
    data: HashMap<String, String>
}

// Methods:
- insert(key, value)
- get(key) -> Option<&String>
- remove(key) -> Option<String>
- contains_key(key) -> bool
```

**Concepts**: Ownership, borrowing, HashMap usage

---

### 02. Transaction Validator
**Goal**: Practice structs, enums, and error handling

**What to build**:
```rust
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    signature: Option<String>
}

enum ValidationError {
    InvalidAmount,
    InvalidAddress,
    MissingSignature
}

fn validate_transaction(tx: &Transaction) -> Result<(), ValidationError>
```

**Concepts**: Enums, Result types, pattern matching

---

### 03. Generic Merkle Node
**Goal**: Learn generics and traits

**What to build**:
```rust
trait Hashable {
    fn hash(&self) -> String;
}

struct MerkleNode<T: Hashable> {
    data: T,
    hash: String
}
```

**Concepts**: Traits, generics, trait bounds

---

### 04. Memory Pool (Mempool)
**Goal**: Collections and data structure management

**What to build**:
```rust
struct Mempool {
    pending: Vec<Transaction>,
    max_size: usize
}

// Methods:
- add_transaction(tx)
- get_top_transactions(n) -> Vec<Transaction>
- remove_transaction(tx_id)
- is_full() -> bool
```

**Concepts**: Vec operations, sorting, capacity management

---

## 🔧 Setup

```bash
cd 01-rust-fundamentals
cargo new <project-name>
cd <project-name>
cargo run
```

## 📖 Essential Resources

**Must Read**:
1. [The Rust Book - Chapters 1-10](https://doc.rust-lang.org/book/)
   - Ch 4: Ownership (CRITICAL)
   - Ch 6: Enums and Pattern Matching
   - Ch 10: Generics, Traits, Lifetimes

2. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
   - Work through all examples

**Practice**:
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust)

## 💡 Key Concepts

### Ownership Rules
```rust
// 1. Each value has an owner
let s = String::from("hello");

// 2. Only one owner at a time
let s2 = s; // s is moved, can't use anymore

// 3. When owner goes out of scope, value is dropped
{
    let s3 = String::from("temp");
} // s3 is dropped here
```

### Borrowing
```rust
// Immutable borrow
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop (it's a reference)

// Mutable borrow
fn append(s: &mut String) {
    s.push_str(" world");
}
```

### Error Handling
```rust
// Use Result for recoverable errors
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Use Option for optional values
fn find_user(id: u64) -> Option<User> {
    // Return Some(user) or None
}
```

## ✅ Completion Checklist

Before moving to Phase 2, ensure you can:

- [ ] Explain ownership and borrowing rules
- [ ] Use structs and enums effectively
- [ ] Implement and use traits
- [ ] Handle errors with Result and Option
- [ ] Work with Vec, HashMap, BTreeMap
- [ ] Understand lifetimes in function signatures
- [ ] Use pattern matching confidently

## 🎓 Moving Forward

Once comfortable with these concepts, proceed to **Phase 2: Blockchain Basics** where you'll apply these fundamentals to build a real blockchain!

## 🚨 Common Pitfalls

1. **Fighting the borrow checker**: Understand it's helping you write safe code
2. **Cloning everything**: Learn when to borrow vs clone
3. **Unwrap() in production**: Use proper error handling
4. **Ignoring lifetimes**: They ensure reference validity

## 📝 Practice Tips

1. **Build without looking**: Try implementing projects from memory
2. **Read compiler errors**: They're usually helpful and educational
3. **Refactor**: Rewrite code to be more idiomatic
4. **Review**: Come back to ownership concepts regularly

---

**Time Investment**: 2-3 weeks of focused learning
**Difficulty**: ⭐⭐⭐ (Ownership is challenging but worth it!)
