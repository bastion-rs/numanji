//!
//! [![Numanji](https://raw.githubusercontent.com/bastion-rs/numanji/master/img/numanji.jpg)](https://github.com/bastion-rs/numanji)
//!
//! Local-affinity first NUMA-aware allocator with optional fallback.
//!
//! This crate supplies NUMA-aware local policy enabled allocation.
//!
//! ### When using `autoselect`
//! Fallback system is triggered with `autoselect`.
//! If system is not supporting NUMA-aware allocation it falls back to `Jemalloc`.
//!
//! ### When using `NUMA-aware`
//! If autoselect is not used, `memmap` fallback will be triggered with
//! default system page size and it will be used as allocator.
//!
//! # Examples
//!
//! ```rust
//! // Allocator generator macro
//! #![feature(allocator_api)]
//! #![feature(nonnull_slice_from_raw_parts)]
//! use numanji::*;
//!
//! // Do autoselect for allocator
//! autoselect!();
//!
//! // Allocated by Numanji based on your Numa availability on your system.
//! let _vec = Vec::<usize>::with_capacity(1234);
//! ```
//!

#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]

mod autoselect;
mod nonnuma_allocator;
mod numa_aware_allocator;

pub mod prelude {
    pub use super::autoselect;
    pub use super::nonnuma_allocator;
    pub use super::numa_aware_allocator;
}
