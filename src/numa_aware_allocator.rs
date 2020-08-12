#[macro_export]
macro_rules! numa_aware_allocator {
  () => {
        // General imports
        use allocator_suite::adaptors::prelude::*;
        use core::ptr::NonNull;
        use std::alloc::{System, GlobalAlloc, AllocRef, Layout, AllocErr};
        use allocator_suite::memory_sources::mmap::memory_map_source::MemoryMapSource;
        use allocator_suite::extensions::usize_ext::UsizeExt;
        use allocator_suite::allocators::allocator::Allocator;
        use allocator_suite::memory_address::MemoryAddress;
        use allocator_suite::allocators::memory_map_allocator::MemoryMapAllocator;

        use lazy_static::*;

        #[allow(missing_docs)]
        pub fn allocator_instance() -> &'static AllocatorAdaptor<'static, MemoryMapAllocator> {
            lazy_static! {
                static ref MMAP_ALLOC: MemoryMapAllocator = {
                    #[cfg(any(target_os = "android", target_os = "linux"))] {
                        use allocator_suite::memory_sources::mmap::numa::numa_settings::NumaSettings;
                        use allocator_suite::memory_sources::mmap::numa::numa_allocation_policy::NumaAllocationPolicy;

                        let numa_settings = NumaSettings::new(NumaAllocationPolicy::Local, false);
                        let mmap = MemoryMapSource::with_numa_settings(numa_settings);
                        MemoryMapAllocator(mmap)
                    }

                    #[cfg(not(any(target_os = "android", target_os = "linux")))] {
                        let mmap = MemoryMapSource::default();
                        MemoryMapAllocator(mmap)
                    }
                };


                static ref MMAP_ADAPTER: AllocatorAdaptor<'static, MemoryMapAllocator> = {
                    MMAP_ALLOC.adapt()
                };
            }

            &*MMAP_ADAPTER
        }

        #[allow(missing_docs)]
        #[derive(Debug, Copy, Clone)]
        pub struct NumaAllocator;

        #[allow(missing_docs)]
        #[global_allocator]
        pub static GLOBAL: NumaAllocator = NumaAllocator;

        unsafe impl Sync for NumaAllocator {}

        unsafe impl GlobalAlloc for NumaAllocator {
            #[inline(always)]
            unsafe fn alloc(&self, layout: Layout) -> *mut u8
            {
                allocator_instance().global_alloc_alloc(layout)
            }

            #[inline(always)]
            unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
            {
                allocator_instance().global_alloc_dealloc(ptr, layout)
            }

            #[inline(always)]
            unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
            {
                allocator_instance().global_alloc_alloc_zeroed(layout)
            }

            #[inline(always)]
            unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
            {
                allocator_instance().global_alloc_realloc(ptr, layout, new_size)
            }
        }

        unsafe impl AllocRef for NumaAllocator {
            #[inline(always)]
            fn alloc(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr>
            {
                let size = layout.size();
                let ptr = unsafe { allocator_instance().alloc_alloc_zeroed(layout) }?;
                Ok(NonNull::slice_from_raw_parts(ptr, size))
            }

            #[inline(always)]
            unsafe fn dealloc(&mut self, ptr: MemoryAddress, layout: Layout)
            {
                allocator_instance().alloc_dealloc(ptr, layout)
            }
        }
  }
}
