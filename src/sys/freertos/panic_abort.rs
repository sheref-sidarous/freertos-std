use crate::sys::freertos::freertos_api;
use core::panic::PanicPayload;
use core::any::Any;

#[no_mangle]
pub unsafe extern "C" fn __rust_panic_cleanup(_: *mut u8) -> *mut (dyn Any + Send + 'static) {
    unreachable!()
}

#[no_mangle]
pub unsafe fn __rust_start_panic(payload: &mut dyn PanicPayload) -> u32 {
    freertos_api::rust_std_vAssertCalled();
}