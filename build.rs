#[cfg(any(feature = "openblas-system", feature = "netlib-system"))]
fn main() {
    // if we are not on arch, exist
    if !std::path::Path::new("/etc/arch-release").exists() {
        return;
    }

    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=cblas");
}

#[cfg(not(any(feature = "openblas-system", feature = "netlib-system")))]
fn main() {}
