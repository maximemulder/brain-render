// build.rs
fn main() {
    // Tell Rust about our custom cfg
    println!("cargo::rustc-check-cfg=cfg(web_sys_unstable_apis)");

    // Always enable web_sys_unstable_apis since we need it for WebGPU
    println!("cargo::rustc-cfg=web_sys_unstable_apis");
}
