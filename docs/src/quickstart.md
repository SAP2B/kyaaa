# Book macro

## Examples

### quick.rs
```rust
{{#include ../../examples/quick.rs}}
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

