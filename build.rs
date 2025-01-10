use std::env;
use std::fs;
use std::path::PathBuf;

// This is not needed anymore since we are using the include_bytes macro
// I still leave it here, in case you want to use it in the future
fn main() {
    // OUT_DIR is automatically set by cargo and contains the build directory path
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let assets_dir = PathBuf::from("assets");

    // Create the destination directory in the build output
    let dest_path = out_path.join("../../../assets");
    fs::create_dir_all(&dest_path).unwrap();

    // Iterate over all files in the assets directory and copy them to the destination
    for entry in fs::read_dir(assets_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            fs::copy(&path, dest_path.join(file_name)).unwrap();
        }
    }

    // Tell cargo to re-run the build script whenever any file in the assets directory changes
    println!("cargo:rerun-if-changed=assets/*");
}