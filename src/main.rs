use std::env;
use std::path::PathBuf;

fn main() {
    // The path to the libSTARK library
    let libstark_path = env::var("../libSTARK").unwrap_or("../libSTARK".to_string());

    // Tell Cargo to link the libstark library
    println!("cargo:rustc-link-search=native={}/build/lib", libstark_path);
    println!("cargo:rustc-link-lib=static=stark");

    let include_paths = vec![
        format!("/Documents/libSTARK/algebra/algebralib/headers/algebraLib/"),
      
      //  format!("{}/libstark/deps/common", libstark_path),
    ];
    // Generate the bindings for BairWitness.hpp
    let mut builder = bindgen::Builder::default()
        .header(format!("{}/libstark/src/languages/Bair/BairWitness.hpp", libstark_path))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
        // .generate()
        // .expect("Unable to generate bindings");
    for path in include_paths {
        builder = builder.clang_arg(format!("-I{}", path));
        
       let current_dir = env::current_dir();
       println!("currentttttttttttt{:?}",current_dir);
    }
    let bindings = builder.generate().expect("Unable to generate bindings");
    // Write the bindings to a file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}