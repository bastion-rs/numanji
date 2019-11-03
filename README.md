<div align="center">
  <img src="https://github.com/bastion-rs/numanji/blob/master/img/numanji.jpg"><br>
</div>

Local-affinity first NUMA-aware allocator with optional fallback.

This crate supplies NUMA-aware local policy enabled allocation.

### When using `autoselect`
Fallback system is triggered with `autoselect`.
If system is not supporting NUMA-aware allocation it falls back to `Jemalloc`.

### When using `NUMA-aware`
If autoselect is not used, `memmap` fallback will be triggered with 
default system page size and it will be used as allocator.

```rust
// Allocator generator macro
use numanji::*;

// Do autoselect for allocator
autoselect!();

fn main() {
    // Allocated by Numanji based on your Numa availability on your system.
    let _vec = Vec::<usize>::with_capacity(1234);
}
```
