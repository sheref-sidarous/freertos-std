//! Platform-specific extensions to `std` for UEFI.

#![unstable(feature = "uefi_std", issue = "100499")]

pub mod env;
#[path = "../windows/ffi.rs"]
pub mod ffi;
