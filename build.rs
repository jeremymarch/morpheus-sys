// build.rs
use std::env;
//use std::fs;
use std::path::Path;
//use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the Cargo-provided output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let vendor_dir = Path::new("morpheus");

    if Path::new(vendor_dir).is_dir() {
        println!("The directory exists.");
    } else {
        println!("The directory does not exist or is not a directory.");
    }

    // Tell Cargo to rerun the build script if the Makefile or any C source changes
    //println!("cargo:rerun-if-changed=vendor/Makefile");
    // You might add more rerun-if-changed lines for source files
    //
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        //.write_to_file(Path::new(".").join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Run the 'make' command from the vendor directory
    // You may need to adjust the command arguments based on your specific Makefile
    // let status = Command::new("make")
    //     .current_dir(&vendor_dir) // Change to the directory containing the Makefile
    //     .arg("all") // Or whatever target you need
    //     .env("OUT_DIR", &out_dir) // Pass OUT_DIR to the Makefile if needed
    //     .status()
    //     .expect("Failed to run make command");
    let status = Command::new("./build_mac.sh")
        .current_dir(&vendor_dir) // Change to the directory containing the Makefile
        //.arg("all") // Or whatever target you need
        //.env("OUT_DIR", &out_dir) // Pass OUT_DIR to the Makefile if needed
        .status()
        .expect("Failed to run make command");

    if !status.success() {
        panic!("make command failed");
    }

    // Tell Cargo which libraries to link and where to find them
    // This assumes your Makefile outputs a library file like 'libfoo.a' into OUT_DIR
    //println!("Looking for library files in {:?}", vendor_dir.join("src").join("anal").display());
    // let full_path =
    //     //Path::new("/")
    //     // .join("Users")
    //     // .join("jeremy")
    //     // .join("Documents")
    //     // .join("code")
    //     Path::new("..")
    //     .join("morpheus")
    //     .join("src")
    //     .join("anal");
    let lib_path = vendor_dir.join("src").join("anal");
    if !lib_path.exists() {
        panic!("Library path not found at {:?}", lib_path);
    }

    //requires the lib prefix to link properly
    //fs::copy(full_path.join("anal.a"), full_path.join("libanal.a")).unwrap();

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=philolog"); // Link against the 'foo' library

    // Tell Cargo where to find the library and how to link it
    // The name "cpfapack" should match the library name after "lib" (e.g., libcpfapack.a)
    //println!("cargo:rustc-link-search=native={}", out_path.join("c_interface").display());
    //println!("cargo:rustc-link-lib=static=cpfapack"); // Links the static library

    // Invalidate the build if the C source files or Makefile change
    println!("cargo:rerun-if-changed=c_src_folder/Makefile");
    println!("cargo:rerun-if-changed=c_src_folder/src/*.c");
}
