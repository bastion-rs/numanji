#[macro_export]
macro_rules! nonnuma_allocator {
    () => {
        #[allow(missing_docs)]
        #[global_allocator]
        pub static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
    };
}
