use crate::alloc::{GlobalAlloc, Layout, System};

use crate::sys::freertos::freertos_api;



fn can_use_system_alignment(align_req : usize) -> bool {
    let system_alignment = unsafe{ freertos_api::rust_std_get_portBYTE_ALIGNMENT() };

    if system_alignment >= align_req && align_req % system_alignment == 0 {
        true
    } else {
        false
    }

}


#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        if can_use_system_alignment(layout.align()) {
            // system allocator can directly satisfy the alignment requirement,
            // no need for manual alignment
            return unsafe { freertos_api::rust_std_pvPortMalloc(layout.size() as u32) };
        }

        // Manual alignment is necessary
        let size_to_alloc = layout.size() + layout.align();
        let allocated_ptr = unsafe {
            freertos_api::rust_std_pvPortMalloc(size_to_alloc as u32)
        };

        // find padding and aligned_pointer
        // make sure that at least one byte is available in padding to store
        // the padding value (hence the .offset(1))
        let padding = allocated_ptr.offset(1).align_offset(layout.align()) + 1;
        let aligned_ptr = allocated_ptr.offset(padding as isize);

        // store padding just before the aligned_ptr
        unsafe {
            *aligned_ptr.offset(-1) = padding as u8;
        }

        aligned_ptr
    }

    #[inline]
    unsafe fn dealloc(&self, aligned_ptr: *mut u8, layout: Layout) {

        let ptr_to_dealloc = match can_use_system_alignment(layout.align()) {
            true => aligned_ptr,
            false => {
                let padding = *aligned_ptr.offset(-1) as isize;
                let original_ptr = aligned_ptr.offset(-1 * padding);
                original_ptr
            }
        };

        unsafe {
            freertos_api::rust_std_vPortFree(ptr_to_dealloc);
        }
    }

}
