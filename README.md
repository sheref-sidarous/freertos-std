# A Rust Standard Library implementation for FreeRTOS

This is an early alpha version of the Rust Standard Library port for FreeRTOS. So far, it has basic support of threads and sync primitives.

## How to use
It is recommended to include this repo as a submodule in your workspace, this is because the FreeRTOS build system needs to include c files from it.
```
$ git submodule add git@github.com:sheref-sidarous/freertos-std.git
```

### rust app:
* Add `freertos-std` dependendcy as the `std` library in Cargo.toml
```
std = {path = "../freertos-std", features = ["panic_immediate_abort"], package = "freertos-std"}
```
Where path refers to where `freertos-std` were checked out

* Build your app as a staticlib
```
[lib]
crate-type = ["staticlib"]
```

* You will need to include the `restricted_std` feature in the top of your root `lib.rs`
```
#![feature(restricted_std)]
```

### FreeRTOS build system:
* Build the C file `src/sys/freertos/rust_std_shim.c` from freertos-std into the build system
* Link the Rust app built static library

## Example usage
Check out [freertos-std-example](https://github.com/sheref-sidarous/freertos-std-example) for an example that builds and runs this library using Qemu