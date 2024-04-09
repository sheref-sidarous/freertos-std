use core::sync::atomic::{AtomicUsize, Ordering};

use crate::pin::Pin;
use crate::time::Duration;
use crate::sys::freertos::freertos_api;

const TOKEN_RELEASED : usize = 0;
const TOKEN_HELD : usize = 1;

pub struct Parker {
    inner : freertos_api::SemaphoreHandle_t,
}

unsafe impl Sync for Parker {}
unsafe impl Send for Parker {}

impl Parker {
    pub unsafe fn new_in_place(parker: *mut Parker) {
        parker.write(Parker {
            inner : freertos_api::rust_std_xSemaphoreCreateBinary(),
        })
    }

    pub unsafe fn park(self: Pin<&Self>) {
        self.park_internal(None)
    }

    pub unsafe fn park_timeout(self: Pin<&Self>, dur: Duration) {
        self.park_internal(Some(dur))
    }

    unsafe fn park_internal(self: Pin<&Self>, dur : Option<Duration>) {

        let timeout = match dur {
            None => freertos_api::portMAX_DELAY,
            Some(value) => freertos_api::rust_std_msec_to_ticks(value.as_millis() as u32)
        };

        freertos_api::rust_std_xSemaphoreTake(self.inner, timeout);
    }

    pub fn unpark(self: Pin<&Self>) {
        unsafe {freertos_api::rust_std_xSemaphoreGive(self.inner)};
    }
}
