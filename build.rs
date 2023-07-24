use std::env;
use cc;

fn main() {
    println!("cargo:rustc-cfg=feature=\"restricted-std\"");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/sys/freertos/rust_std_shim.c");
    println!("cargo:rustc-env=STD_ENV_ARCH={}", env::var("CARGO_CFG_TARGET_ARCH").unwrap());
    let target = env::var("TARGET").expect("TARGET was not set");

    if !target.contains("none") {
        println!("cargo:warning=\"This library is intended for none targets\"");
    }

    //let freertos_config_path = env::var("FREERTOS_CONFIG_PATH").expect("Need to know where is FREERTOS_CONFIG_PATH");
    cc::Build::new()
        .file("src/sys/freertos/rust_std_shim.c")
        .include("/home/shiro/projects/rust/freertos-std-example/FreeRTOS-Kernel/include")
        .include("/home/shiro/projects/rust/freertos-std-example/FreeRTOS-Qemu-Demo")
        .include("/home/shiro/projects/rust/freertos-std-example/FreeRTOS-Kernel/portable/GCC/ARM_CM3")
        .compile("freertos_shim");


}
