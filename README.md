# A Rust Standard Library implementation for FreeRTOS

This is a work-in-progress fork of the Rust Standard Library targeting FreeRTOS. So far, it has basic support of threads and sync primitives.

## Features
* stdout via semi-hosting
* Support for panic abort and panic unwind

## Supported targets / platforms
* `thumbv7m-none-eabi` tested on Qemu's `mps2-an385`

## Used Rust toolchain version
std library is tightly coupled with the Rust compiler, and it uses many internal features. Therefore a nightly toolchain must be used. Moreover, a *specific* nightly has to be used, the one that matches whenever last this library was updated from the upstream std library.
This version is tracked in [supported-toolchain.txt]

## How to use
It is recommended to include this repo as a submodule in your workspace, this is because the FreeRTOS build system needs to include c files from it.
```
$ git submodule add git@github.com:sheref-sidarous/freertos-std.git
```

### rust app:
* Add `freertos-std` dependency as the `std` library in Cargo.toml
```
std = {path = "../freertos-std", package = "freertos-std"}
```
Where path refers to where `freertos-std` were checked out. Use features `panic-unwind` and `stdio-semihosting` if you want to.

* Build your app as a staticlib
```
[lib]
crate-type = ["staticlib"]
```

### FreeRTOS build system:
* Build the C file `src/sys/freertos/rust_std_shim.c` from freertos-std within your FreeRTOS the build system
* Link the Rust app built as a static library

## Panic Unwind
This library supports panic unwind, but to use it you'd need to recompile Rust toolchain to support unwind for your selected target. There is a helper script in [freertos-std-example](https://github.com/sheref-sidarous/freertos-std-example) subfolder `rust-unwind-toolchain` for an example of doing so.
Then in your rust app, enable freertos-std `panic-unwind` feature and add the compiler flag `-Cpanic=unwind`
```
rustflags = ["-Cpanic=unwind"]
```

## Example usage
Check out [freertos-std-example](https://github.com/sheref-sidarous/freertos-std-example) for an example that builds and runs this library using Qemu
