#![allow(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod env;
pub mod fs;
pub mod io;
pub mod locks;
pub mod net;
pub mod once;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
pub mod pipe;
pub mod process;
pub mod stdio;
pub mod thread;
#[cfg(target_thread_local)]
pub mod thread_local_dtor;
pub mod thread_local_key;
pub mod thread_parking;
pub mod time;

mod common;
pub use common::*;

mod freertos_api;

// support for panics
pub mod unwind;
#[cfg(panic_unwind)]
mod panic_unwind;

#[cfg(not(panic_unwind))]
mod panic_abort;
