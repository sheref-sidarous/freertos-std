use crate::sys::locks::Mutex;
use crate::time::Duration;
use crate::sys::freertos::freertos_api;

use crate::cell::UnsafeCell;
use crate::ffi::c_void;
use crate::ptr::null_mut;

use crate::sync::atomic::{
    AtomicPtr, AtomicUsize,
    Ordering::{AcqRel, Acquire, SeqCst},
};

pub struct Condvar {
    waiting_thread_cnt : AtomicUsize,
    blocking_semaphore : AtomicPtr<c_void>,
}

impl Condvar {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Condvar {
        Condvar {
            waiting_thread_cnt : AtomicUsize::new(0),
            blocking_semaphore : AtomicPtr::new(null_mut())
        }
    }

    #[inline]
    pub fn notify_one(&self) {
        if self.decrement_waiting_threads() {
            unsafe {
                freertos_api::rust_std_xSemaphoreGive(self.get_ptr());
            }
        }
    }

    #[inline]
    pub fn notify_all(&self) {
        while self.decrement_waiting_threads() {
            unsafe {
                freertos_api::rust_std_xSemaphoreGive(self.get_ptr());
            }
        }
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        self.increment_waiting_threads();
        mutex.unlock();
        unsafe {
            let r = freertos_api::rust_std_xSemaphoreTake(self.get_ptr(), freertos_api::portMAX_DELAY);
            assert_eq!(r, true, "Timed out waiting for Semaphore");
        }
        mutex.lock();
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        self.increment_waiting_threads();
        mutex.unlock();
        let r = unsafe {
            freertos_api::rust_std_xSemaphoreTake(self.get_ptr(), freertos_api::rust_std_msec_to_ticks(dur.as_millis() as u32))
        };
        mutex.lock();
        r
    }

    fn increment_waiting_threads(&self) {
        let old_count = self.waiting_thread_cnt.fetch_add(1, SeqCst);
        // guard against an overflow
        assert_ne!(old_count, usize::MAX);
    }

    fn decrement_waiting_threads(&self) -> bool {
        let mut count = self.waiting_thread_cnt.load(Acquire);
        while count > 0 {
            match self.waiting_thread_cnt.compare_exchange(count, count - 1, SeqCst, Acquire) {
                Ok(_) => return true,
                Err(updated_count) => {
                    count = updated_count;
                }
            }
        }
        return false;
    }

    fn get_ptr(&self) -> freertos_api::SemaphoreHandle_t {
        let ptr = self.blocking_semaphore.load(Acquire);
        if ptr.is_null() { self.initialize() } else { ptr }
    }

    fn initialize(&self) -> freertos_api::SemaphoreHandle_t {
        let new_ptr = unsafe {
            freertos_api::rust_std_xSemaphoreCreateCounting(freertos_api::UBaseType_t::MAX, 0)
        };
        match self.blocking_semaphore.compare_exchange(null_mut(), new_ptr, AcqRel, Acquire) {
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
