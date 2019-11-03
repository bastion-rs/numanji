#[cfg(not(any(target_os = "android", target_os = "linux")))]
#[macro_export]
macro_rules! autoselect {
    () => {
        use numanji::nonnuma_allocator;
        nonnuma_allocator!();
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[macro_export]
macro_rules! autoselect {
    () => {
        use numanji::numa_aware_allocator;
        numa_aware_allocator!();
    }
}
