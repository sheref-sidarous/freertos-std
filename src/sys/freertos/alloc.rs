use crate::alloc::{GlobalAlloc, Layout, System};
use crate::sys::freertos::semihosting;
use crate::cell::OnceCell;

struct AllocTracer {
    inner : OnceCell<semihosting::HostStream>,
}

unsafe impl Sync for AllocTracer {}

static ALLOC_TRACE : AllocTracer = AllocTracer {inner : OnceCell::new() };

extern "C" {
    pub fn pvPortMalloc( xSize : u32 ) -> *mut u8 ;
    pub fn vPortFree( pv : *mut u8 );
}

impl AllocTracer {
    fn send_trace(&self, trace : &str) {
        let host_stream = self.inner.get_or_init( || {
            semihosting::open("malloc_log.log", 0).unwrap()
        });
        //let host_stream = self.inner.get_mut().unwrap();
        host_stream.write_all(trace.as_bytes()).unwrap();
    }
}


#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        ALLOC_TRACE.send_trace("Hello :)\n");

        let size_to_alloc = layout.size() + layout.align();
        let allocated_ptr = unsafe {
            pvPortMalloc(size_to_alloc as u32)
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
    unsafe fn dealloc(&self, aligned_ptr: *mut u8, _layout: Layout) {
        unsafe {
            let padding = *aligned_ptr.offset(-1) as isize;
            let original_ptr = aligned_ptr.offset(-1 * padding);
            vPortFree(original_ptr);
        }
    }

}
