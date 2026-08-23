# kyaaa 🚀

An ultra-efficient, **zero-cost, `no_std`-compatible Heterogeneous List (HList)** implementation for Rust using declarative macros. It bridges the gap between high-level ergonomic data modeling and bare-metal assembly performance.

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| **Zero Allocation** | Backed entirely by static memory blocks and raw pointers (`*const ()`). No heap allocations (`Box`, `Vec`) — optimal for embedded systems, game engines, drivers, and kernels. |
| **Dual-Access Ergonomics** | Semantic named access (`hlist.zelda()`) combined with dynamic index-based retrieval via an automatically derived `u8` enum (`hlist.get(0)`). |
| **Surgical Mutability** | Fields prefixed with `mut` are selectively wrapped in an `UnsafeCell` with manual `Sync` guarantees, avoiding blanket mutability or heavy synchronization primitives (`Mutex`, `RwLock`). |
| **Zero-Cost Abstraction** | Optimized aggressively by LLVM into direct static memory offsets with **zero runtime overhead** and zero function call instructions (`call`). |

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kyaaa = "0.0.3"
```

---

## 🎮 Zero-Cost Assembly Example (`examples/gameboy.rs`)

This is the exact code used to verify the zero-cost assembly output, featuring mixed structs, named mutability, and optimization barriers:

```rust

use kyaaa::book;

#[repr(transparent)]
pub struct Name(&'static str);

#[repr(transparent)]
pub struct Title(&'static str);

pub struct Game {
    title: Title,
    best_seller: bool,
}

pub struct Boy {
    name: Name,
    dev: bool,
    age: u8,
}

fn main() {
    let hlist = book!(
        nier_automata => Game { title: Title("Nier Automata"), best_seller: true },
        mut zelda => Game { title: Title("Zelda"), best_seller: true },
        sap => Boy { name: Name("SAP2B"), age: 69, dev: true },
        mut john => Boy { name: Name("John Doe"), age: 15, dev: false }
    );

    unsafe {
        hlist.zelda().title = Title("Zelda: Breath of the Wild");
        hlist.zelda().best_seller = false;

        hlist.john().name = Name("John Wick");
        hlist.john().dev = true;
        hlist.john().age = 35;

        black_box(hlist.zelda());
        black_box(hlist.john());
    }

    black_box(hlist.nier_automata());
    black_box(hlist.sap());
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
```
---

## 🔬 Assembly Proof (Zero-Cost Verified)

Inspecting the optimized release assembly via `cargo asm --example gameboy --release gameboy::main`:

![Assembly Proof](assets/gameboy_asm.png)

### Why This Architecture is a Masterpiece:
1. **Zero Function Calls:** The `call` instruction is completely absent. Accessor methods are aggressively inlined by LLVM.
2. **Direct Static Memory Offsets:** String pointer updates (`+8`), boolean/integer modifications (`+16`), and property writes target static memory offsets directly relative to the instruction pointer (`rip`).
3. **Pure Machine Efficiency:** Zero allocators, zero locks, zero hidden vtables—just raw, bare-metal pointer manipulation operating at maximum processor velocity.

---

## 🔍 Advanced Features: Dynamic Index Retrieval (`get`)

If you need to fetch items dynamically by index or write unit tests, `kyaaa` automatically maps every element to an internal `u8` index via `get(u8)`:

```rust
#[test]
fn test_dynamic_retrieval() {
    let hlist = book!(
        nier_automata => Game { title: Title("Nier Automata"), best_seller: true },
        mut zelda => Game { title: Title("Zelda"), best_seller: true }
    );

    let game_at_0: &Game = hlist.get(0);
    let game_at_1: &Game = hlist.get(1);

    assert_eq!(game_at_0.best_seller, true);
    
    hlist.zelda().best_seller = false;
    assert_eq!(hlist.get::<Game>(1).best_seller, false);
}
```

---

## ⚖️ License

Licensed under the **GNU Affero General Public License v3.0 (AGPLv3)**.

