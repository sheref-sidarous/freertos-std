#![allow(non_upper_case_globals)]

use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::Duration;
use crate::sys::freertos::freertos_api;
use crate::ffi::{c_void, c_char};
use crate::collections::HashMap;
use crate::boxed::Box;
use crate::ptr::null_mut;
use super::thread_local_key::thread_exit_tls_cleaner;

pub struct Thread {
    handle : freertos_api::TaskHandle_t,
    join_semaphore : freertos_api::SemaphoreHandle_t,
}
struct ThreadDescriptor {
    entry : Box<dyn FnOnce()>,
    join_semaphore : freertos_api::SemaphoreHandle_t,
}

pub const DEFAULT_MIN_STACK_SIZE: usize = 1024;

#[allow(non_camel_case_types)]
type TickType_t = u32;


extern "C" fn thread_entry (arg : *mut c_void) {


    let join_semaphore = {

        let list_kept_ptr = unsafe {
            // create the Vector for TLS
            let list : *mut Vec<*mut u8> = Box::into_raw(Box::new(Vec::new()));
            freertos_api::rust_std_vTaskSetThreadLocalStoragePointer(
                null_mut(), 0, list.clone() as *mut c_void);
            list
        };

        let thread_descriptor = unsafe { Box::from_raw(arg as *mut ThreadDescriptor) };

        (thread_descriptor.entry)();

        unsafe {

            let tls_list : Box<Vec<*mut u8>> = Box::from_raw(list_kept_ptr);
            thread_exit_tls_cleaner(tls_list);

        }

        thread_descriptor.join_semaphore
    };

    unsafe {
        freertos_api::rust_std_xSemaphoreGive(join_semaphore);
        freertos_api::rust_std_vTaskDelete( core::ptr::null_mut() );
    }
}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {

        let join_semaphore = freertos_api::rust_std_xSemaphoreCreateBinary();

        if join_semaphore == core::ptr::null_mut() {
            // TODO : proper error code here
            return io::Result::Err(io::Error::from_raw_os_error(0));
        }

        let arg : *mut ThreadDescriptor= Box::into_raw(Box::new(ThreadDescriptor {
            entry: p,
            join_semaphore : join_semaphore.clone(),
        }));

        let mut thread_handle : freertos_api::TaskHandle_t = core::ptr::null_mut();

        let r = freertos_api::xTaskCreate(
            thread_entry,  /* entry point */
            core::ptr::null(),  /* Task name */
            stack as freertos_api::configSTACK_DEPTH_TYPE, /*  */
            arg as *mut c_void, freertos_api::DefaultTaskPriority, /* */
            &mut thread_handle as *mut freertos_api::TaskHandle_t); /* get the handle back here */

        if r == freertos_api::pdPASS {
            // Success !
            io::Result::Ok(Thread {
                handle : thread_handle,
                join_semaphore : join_semaphore,
            })
        } else {
            io::Result::Err(io::Error::from_raw_os_error(r))
        }
    }

    pub fn yield_now() {
        unsafe { freertos_api::rust_std_taskYIELD(); }
    }

    pub fn set_name(_name: &CStr) {
        // ignore name setting
    }

    pub fn sleep(dur: Duration) {
        unsafe {
            freertos_api::vTaskDelay(
                freertos_api::rust_std_msec_to_ticks(dur.as_millis() as u32))
        }
    }

    pub fn join(self) {
        unsafe {
            freertos_api::rust_std_xSemaphoreTake(self.join_semaphore, freertos_api::portMAX_DELAY);
        }
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsafe { io::Result::Ok(NonZeroUsize::new_unchecked(1)) }
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
