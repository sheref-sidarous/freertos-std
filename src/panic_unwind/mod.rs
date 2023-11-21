//! Implementation of panics via stack unwinding
//!
//! This crate is an implementation of panics in Rust using "most native" stack
//! unwinding mechanism of the platform this is being compiled for. This
//! essentially gets categorized into three buckets currently:
//!
//! 1. MSVC targets use SEH in the `seh.rs` file.
//! 2. Emscripten uses C++ exceptions in the `emcc.rs` file.
//! 3. All other targets use libunwind/libgcc in the `gcc.rs` file.
//!
//! More documentation about each implementation can be found in the respective
//! module.




// `real_imp` is unused with Miri, so silence warnings.
#![cfg_attr(miri, allow(dead_code))]

use crate::boxed::Box;
use core::any::Any;
use core::panic::BoxMeUp;


#[path = "gcc.rs"]
mod real_imp;


cfg_if::cfg_if! {
    if #[cfg(miri)] {
        // Use the Miri runtime.
        // We still need to also load the normal runtime above, as rustc expects certain lang
        // items from there to be defined.
        #[path = "miri.rs"]
        mod imp;
    } else {
        // Use the real runtime.
        use real_imp as imp;
    }
}

extern "C" {
    /// Handler in std called when a panic object is dropped outside of
    /// `catch_unwind`.
    fn __rust_drop_panic() -> !;

    /// Handler in std called when a foreign exception is caught.
    fn __rust_foreign_exception() -> !;
}

#[rustc_std_internal_symbol]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn __rust_panic_cleanup(payload: *mut u8) -> *mut (dyn Any + Send + 'static) {
    Box::into_raw(imp::cleanup(payload))
}

// Entry point for raising an exception, just delegates to the platform-specific
// implementation.
//#[rustc_std_internal_symbol]
#[no_mangle]
pub unsafe fn __rust_start_panic(payload: &mut dyn BoxMeUp) -> u32 {
    let payload = Box::from_raw(payload.take_box());

    imp::panic(payload)
}
