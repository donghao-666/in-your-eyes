static PRIVATE_FRAMEWORKS_PATH: &'static str = "/System/Library/PrivateFrameworks";

fn main() {
    println!(
        "cargo:rustc-link-search=framework={path}",
        path = PRIVATE_FRAMEWORKS_PATH
    );
    // println!("cargo:rustc-link-lib=framework={framework}", framework="Foundation");
    // println!("cargo:rustc-link-lib=framework={framework}", framework="AppKit");
    println!(
        "cargo:rustc-link-lib=framework={framework}",
        framework = "DisplayServices"
    );
}
