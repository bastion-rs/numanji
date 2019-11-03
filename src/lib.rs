//!
//! [![Numanji](https://raw.githubusercontent.com/bastion-rs/numanji/master/img/numanji.jpg)](https://github.com/bastion-rs/numanji)
//!
//! Local-affinity first NUMA-aware allocator with optional fallback.
//!
//! # Examples
//!
//! ```rust
//! // Allocator generator macro
//! use numanji::*;
//!
//! // Do autoselect for allocator
//! autoselect!();
//!
//! fn main() {
//!     // Allocated by Numanji based on your Numa availability on your system.
//!     let _vec = Vec::<usize>::with_capacity(1234);
//! }
//! ```
//!

#![feature(allocator_api)]

mod numa_aware_allocator;
mod nonnuma_allocator;
mod autoselect;

pub mod prelude {
    pub use super::numa_aware_allocator;
    pub use super::nonnuma_allocator;
    pub use super::autoselect;
}
