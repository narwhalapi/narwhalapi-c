//! Give the shared library a stable install name so installed programs find it via rpath.
fn main() {
    match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("macos") => {
            println!("cargo:rustc-cdylib-link-arg=-Wl,-install_name,@rpath/libnarwhalapi.dylib")
        }
        Ok("linux") => println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libnarwhalapi.so.0"),
        _ => {}
    }
}
