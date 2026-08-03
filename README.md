# 🦀 Rust Foundations & Redis KV Store (`rustKVStore`)

A thorough, step-by-step journey learning Rust from absolute ground zero before building a Redis-like Key-Value store.

---

## 📚 Rust Book Foundations Curriculum

| Section | Topic | Key Concepts | Status |
| :--- | :--- | :--- | :---: |
| **Foundations 1** | **Chapter 3: Control Flow & Types** | Shadowing, Tuples, `if` expressions, `for` loops | 🔄 *In Progress* |
| **Foundations 2** | **Chapter 4: Ownership & Borrowing** | Move semantics, Borrowing (`&` vs `&mut`), References, Slices | ⏳ *Pending* |
| **Foundations 3** | **Chapter 5: Structs & Methods** | `struct`, `impl`, `self`, `&self`, `&mut self`, Associated functions | ⏳ *Pending* |
| **Foundations 4** | **Chapter 6: Enums & Pattern Matching** | `enum`, `Option<T>`, `match`, `if let` | ⏳ *Pending* |

---

## 🗺️ Key-Value Store Project Roadmap (Post-Foundations)

| Module | Topic | Description | Status |
| :--- | :--- | :--- | :---: |
| **KV Module 1** | **In-Memory KV Store** | HashMap-backed storage with `set`, `get`, `remove` | ⏳ *Pending* |
| **KV Module 2** | **Key Expiration & TTL** | Timestamps, lazy & active key deletion | ⏳ *Pending* |
| **KV Module 3** | **Extended Data Types** | Lists (`VecDeque`), Sets (`HashSet`) | ⏳ *Pending* |
| **KV Module 4** | **Persistence** | Append-Only File (AOF) logging & snapshotting | ⏳ *Pending* |
| **KV Module 5** | **RESP Protocol Server** | Custom Redis Serialization Protocol & Async TCP server | ⏳ *Pending* |
| **KV Module 6** | **Concurrency** | Shared state across threads with `Arc<Mutex<T>>` | ⏳ *Pending* |

---

## 🛠️ LunarVim Shortcuts & Tooling

- `Space + r + t`: Run `cargo test` in a vertical split terminal
- `Ctrl + h` / `Ctrl + l`: Switch between code and terminal split
- `Space + c`: Close current tab / window
