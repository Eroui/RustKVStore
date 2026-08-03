# 🦀 Rust Redis-like Key-Value Store (`rustKVStore`)

A step-by-step journey learning Rust from absolute ground zero by building a Redis-like Key-Value store.

---

## 📌 Learning Journey & Module Roadmap

| Module | Topic | Concepts | Status |
| :--- | :--- | :--- | :---: |
| **Module 0** | **Absolute Basics** | Variables (`let` vs `let mut`), Functions, Return Expressions, Strings | ✅ *Completed* |
| **Module 1** | **Structs & HashMap (In-Memory KV Store)** | `struct`, `impl`, `HashMap`, Ownership, `Option<T>` | 🔄 *In Progress* |
| **Module 2** | **Key Expiration & TTL** | `Instant`, Timestamps, Enums, Pattern Matching | ⏳ *Pending* |
| **Module 3** | **Extended Data Structures** | `VecDeque` (Lists), `HashSet` (Sets) | ⏳ *Pending* |
| **Module 4** | **Persistence (AOF & Files)** | File I/O (`std::fs`), Buffer Writers (`BufWriter`), `Result<T, E>` | ⏳ *Pending* |
| **Module 5** | **Networking & RESP Protocol** | `tokio`, Async/Await, Parsing Byte Streams | ⏳ *Pending* |
| **Module 6** | **Concurrency & Thread Safety** | `Arc<Mutex<T>>`, Shared State, Concurrent Clients | ⏳ *Pending* |

---

## 🛠️ LunarVim Navigation Cheatsheet

### Window Splits
- `:vs` or `:vsplit <filename>` – Open vertical split.
- `:sp` or `:split <filename>` – Open horizontal split.
- `Ctrl + h` – Move cursor to left window.
- `Ctrl + l` – Move cursor to right window.
- `Ctrl + j` – Move cursor to window below.
- `Ctrl + k` – Move cursor to window above.

### Cargo Commands in Vertical Split
- `Space + r + t` – Run `cargo test` in vertical split terminal.
- `Space + r + r` – Run `cargo run` in vertical split terminal.
- `Space + r + c` – Run `cargo check` in vertical split terminal.

---

## 📚 Recommended Reading
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
  - **Chapter 3**: Variables, Mutability, and Basic Functions
  - **Chapter 4**: Understanding Ownership
  - **Chapter 5**: Using Structs to Structure Related Data
  - **Chapter 8.1 & 8.3**: Vectors & HashMaps
