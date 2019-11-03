#[macro_export]
macro_rules! nonnuma_allocator {
    () => {
        #[global_allocator]
        pub static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
    }
}
