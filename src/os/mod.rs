//! OS-specific functionality.

#![stable(feature = "os", since = "1.0.0")]
#![allow(missing_docs, nonstandard_style, missing_debug_implementations)]

pub mod raw;

// The code below could be written clearer using `cfg_if!`. However, the items below are
// publicly exported by `std` and external tools can have trouble analysing them because of the use
// of a macro that is not vendored by Rust and included in the toolchain.
// See https://github.com/rust-analyzer/rust-analyzer/issues/6038.

// On certain platforms right now the "main modules" modules that are
// documented don't compile (missing things in `libc` which is empty),
// so just omit them with an empty module and add the "unstable" attribute.

// Unix, linux, wasi and windows are handled a bit differently.
#[cfg(all(
    doc,
    any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        all(target_vendor = "fortanix", target_env = "sgx")
    )
))]
#[unstable(issue = "none", feature = "std_internals")]
pub mod unix {}
#[cfg(all(
    doc,
    any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        all(target_vendor = "fortanix", target_env = "sgx")
    )
))]
#[unstable(issue = "none", feature = "std_internals")]
pub mod linux {}
#[cfg(all(
    doc,
    any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        all(target_vendor = "fortanix", target_env = "sgx")
    )
))]
#[unstable(issue = "none", feature = "std_internals")]
pub mod wasi {}
#[cfg(all(
    doc,
    any(
        all(target_arch = "wasm32", not(target_os = "wasi")),
        all(target_vendor = "fortanix", target_env = "sgx")
    )
))]
#[unstable(issue = "none", feature = "std_internals")]
pub mod windows {}
