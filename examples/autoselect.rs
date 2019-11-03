// Allocator generator macro
use numanji::*;

// Do autoselect for allocator
autoselect!();

fn main() {
    // Allocated by Numanji based on your Numa availability on your system.
    let _vec = Vec::<usize>::with_capacity(1234);
}
