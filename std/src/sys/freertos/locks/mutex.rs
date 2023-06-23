use crate::sys::freertos::freertos_api;
use crate::cell::UnsafeCell;
use crate::ffi::c_void;
use crate::ptr::null_mut;

//use crate::mem::{forget, MaybeUninit};
//use crate::sys::cvt_nz;
//use crate::sys_common::lazy_box::{LazyBox, LazyInit};
//use crate::ptr;

use crate::sync::atomic::{
    AtomicPtr,
    Ordering::{AcqRel, Acquire},
};

pub struct Mutex {
    ptr: AtomicPtr<c_void>,
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {} // no threads on this platform


impl Mutex {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Mutex {
            Mutex { ptr : AtomicPtr::new(null_mut()) }
    }

    #[inline]
    pub fn lock(&self) {
        unsafe {
            let r = freertos_api::rust_std_xSemaphoreTake(self.get_ptr(), freertos_api::portMAX_DELAY);
            assert_eq!(r, true, "Timed out waiting for Semaphore");
        }
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        unsafe {
            freertos_api::rust_std_xSemaphoreGive(self.get_ptr());
        }
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        unsafe {
            freertos_api::rust_std_xSemaphoreTake(self.get_ptr(), 0)
        }
    }

    fn get_ptr(&self) -> freertos_api::SemaphoreHandle_t {
        let ptr = self.ptr.load(Acquire);
        if ptr.is_null() { self.initialize() } else { ptr }
    }

    fn initialize(&self) -> freertos_api::SemaphoreHandle_t {
        let new_ptr = unsafe {
            freertos_api::rust_std_xSemaphoreCreateMutex()
        };
        match self.ptr.compare_exchange(null_mut(), new_ptr, AcqRel, Acquire) {
            Ok(_) => new_ptr,
            Err(ptr) => {
                // Lost the race to another thread.
                // we should delete the semaphore we've just created and use the one we got back from the compare_exchange
                unsafe {
                    freertos_api::rust_std_vSemaphoreDelete(new_ptr);
                };
                ptr
            }
        }

    }
}
