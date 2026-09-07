# Book macro

## Quick Start
```rust
{{#include ../../examples/quick.rs}}
```

## Examples
### book.rs
```rust
{{#include ../../examples/book.rs}}
```

### gameasm.rs

```rust
{{#include ../../examples/gameasm.rs}}
```
#### assembly proof
```bash
cargo asm --example gameasm --release k_main
```
```toml
{{#include ../../.cargo/config.toml}}

{{#include ../../Cargo.toml}}
```
#### output
![examples/gameasm.rs](./assets/gameasm.png)

