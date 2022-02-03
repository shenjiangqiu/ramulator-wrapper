use cmake;
fn main() {
    // if cargo build debug, then build debug version
    // if cargo build release, then build release version
    let dst = cmake::Config::new("ramulator").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ramulator");
    // if using gcc, link stdc++, if using clang, link libc++
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    }


}
