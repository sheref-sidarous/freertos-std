#![unstable(feature = "panic_unwind", issue = "32837")]
#![feature(link_cfg)]
#![feature(staged_api)]
#![feature(c_unwind)]
#![feature(cfg_target_abi)]
//#![feature(strict_provenance)]
#![cfg_attr(not(target_env = "msvc"), feature(libc))]
#![allow(internal_features)]


mod libunwind;
pub use libunwind::*;


// When building with crt-static, we get `gcc_eh` from the `libc` crate, since
// glibc needs it, and needs it listed later on the linker command line. We
// don't want to duplicate it here.
#[cfg(all(
    target_os = "linux",
    any(target_env = "gnu", target_env = "uclibc"),
    not(feature = "llvm-libunwind"),
    not(feature = "system-llvm-libunwind")
))]
#[link(name = "gcc_s", cfg(not(target_feature = "crt-static")))]
extern "C" {}

#[cfg(all(
    target_os = "linux",
    any(target_env = "gnu", target_env = "uclibc"),
    not(feature = "llvm-libunwind"),
    feature = "system-llvm-libunwind"
))]
#[link(name = "unwind", cfg(not(target_feature = "crt-static")))]
extern "C" {}

#[cfg(target_os = "redox")]
#[link(name = "gcc_eh", kind = "static", modifiers = "-bundle", cfg(target_feature = "crt-static"))]
#[link(name = "gcc_s", cfg(not(target_feature = "crt-static")))]
extern "C" {}

#[cfg(all(target_vendor = "fortanix", target_env = "sgx"))]
#[link(name = "unwind", kind = "static", modifiers = "-bundle")]
extern "C" {}

#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
#[link(name = "gcc_s")]
extern "C" {}

#[cfg(all(target_os = "openbsd", target_arch = "sparc64"))]
#[link(name = "gcc")]
extern "C" {}

#[cfg(all(target_os = "openbsd", not(target_arch = "sparc64")))]
#[link(name = "c++abi")]
extern "C" {}

#[cfg(any(target_os = "solaris", target_os = "illumos"))]
#[link(name = "gcc_s")]
extern "C" {}

#[cfg(target_os = "dragonfly")]
#[link(name = "gcc_pic")]
extern "C" {}

#[cfg(target_os = "haiku")]
#[link(name = "gcc_s")]
extern "C" {}

#[cfg(target_os = "aix")]
#[link(name = "unwind")]
extern "C" {}

#[cfg(target_os = "nto")]
#[link(name = "gcc_s")]
extern "C" {}

#[cfg(target_os = "hurd")]
#[link(name = "gcc_s")]
extern "C" {}
