use std::path::PathBuf;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("lib").canonicalize().expect("cannot canonicalize path");
    println!("cargo:rustc-link-search=native={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=robotcontrol");
}
