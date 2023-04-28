use std::env;
use std::path::PathBuf;
use bindgen;

fn main() {
    // println!("cargo:rustc-link-lib=/home/neeraj/Documents/libSTARK/bin/algebralib/libalgebralib.a");
    // println!("cargo:rustc-link-search=native=/home/neeraj/Documents/libSTARK/bin/algebralib/libalgebralib.a");

    let mut builder = bindgen::Builder::default()
        .header("../libSTARK/libstark/src/languages/Bair/BairWitness.hpp")
        .clang_arg("-I../libSTARK/algebra/algebralib/headers/algebraLib")
        .clang_arg("-I../libSTARK/algebra/FFT/src")
        .clang_arg("-I../libSTARK/libstark/src/")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder.generate().unwrap().write_to_file(out_path.join("./bair_wit.rs")).unwrap();
}








// // extern crate bindgen;
// // use std::env;
// // use std::path::PathBuf;

// // fn main() {
// //     // The path to the libSTARK library
// //     let libstark_path = env::var("../libSTARK").unwrap_or("../libSTARK".to_string());

// //     // Tell Cargo to link the libstark library
// //     println!("cargo:rustc-link-search=native={}/build/lib", libstark_path);
// //     println!("cargo:rustc-link-lib=static=stark");

// //     let include_paths = vec![
// //        // format!("/home/neeraj/Documents/libSTARK/algebra/algebralib/headers/algebraLib"),
// //       format!("./"),
// //       //  format!("{}/libstark/deps/common", libstark_path),
// //     ];
// //     // Generate the bindings for BairWitness.hpp
// //     let mut builder = bindgen::Builder::default()
// //         .header(format!("{}/libstark/src/languages/Bair/BairWitness.hpp", libstark_path))
// //         .parse_callbacks(Box::new(bindgen::CargoCallbacks));
// //         // .generate()
// //         // .expect("Unable to generate bindings");
// //     for path in include_paths {
// //         builder = builder.clang_arg(format!("-I{}", path));
        
// //        let current_dir = env::current_dir();
// //        println!("currentttttttttttt{:?}",current_dir);
// //     }
// //     let bindings = builder.generate().expect("Unable to generate bindings");
// //     // Write the bindings to a file
// //     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
// //     bindings
// //         .write_to_file(out_path.join("bindings.rs"))
// //         .expect("Couldn't write bindings!");
// // }


// use std::env;
// use std::fs;
// use std::path::{Path, PathBuf};
// use bindgen;

// fn main() {
//     // Build libSTARK using `make j8`
//     // let output = std::process::Command::new("make")
//     //     .arg("j8")
//     //     .output()
//     //     .expect("Failed to run make command");

//     // if !output.status.success() {
//     //     panic!("make command failed with exit code {:?}", output.status);
//     // }

//     // Extract all *.o files from the bin folder
//     let bin_dir = Path::new("/home/neeraj/Documents/libSTARK/bin");
//     let object_files = fs::read_dir(bin_dir)
//         .expect("Failed to read bin directory")
//         .filter_map(Result::ok)
//         .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "o"))
//         .map(|entry| entry.path())
//         .collect::<Vec<PathBuf>>();

//     // Generate Rust bindings for BairWitness.hpp using bindgen
//     let mut builder = bindgen::Builder::default()
//         .header("../libSTARK/libstark/src/languages/Bair/BairWitness.hpp")
//         .clang_args(&["-I../libSTARK/algebra/algebralib/headers/algebraLib", "-I../libSTARK/libstark/src/languages/Bair"]);

//     for object_file in object_files.iter() {
//         builder = builder.clang_arg(format!("-L{}", object_file.display()));
//     }

//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     builder
//         .generate()
//         .unwrap()
//         .write_to_file(out_path.join("bair_wit.rs"))
//         .unwrap();
// }