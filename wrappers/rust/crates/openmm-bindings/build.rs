use std::env;
use std::path::PathBuf;

use bindgen;
use cmake::Config;

fn cmake_and_build() -> PathBuf {
    let path = Config::new("external")
        .define("OPENMM_BUILD_PYTHON_WRAPPERS", "OFF")
        .define("OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS", "OFF")
        .define("OPENMM_BUILD_STATIC_LIB", "OFF")
        .define("OPENMM_BUILD_SHARED_LIB", "ON")
        .build();

    println!("cargo:include={}/include", path.display());
    println!("cargo:lib={}/lib", path.display());
    println!("cargo:rustc-link-search=native={}/lib", path.display());
    println!("cargo:rustc-link-lib=dylib=OpenMM");

    return path;
}

fn do_bindgen(include: PathBuf) {
    let header = include.join("OpenMM.h").display().to_string();

    println!("cargo:rerun-if-changed={}", header);

    let bindings = bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=gnu++11")
        .clang_arg(format!("-I{}", include.display()))
        .enable_cxx_namespaces()
        .rustfmt_bindings(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .array_pointers_in_arguments(true)
        .whitelist_function("OpenMM::.*")
        .whitelist_var("OpenMM::.*")
        .whitelist_type("OpenMM::.*")
        .opaque_type(".*")
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    println!("cargo:bindings={}", bindings_path.display());
}

fn main() {
    let path = cmake_and_build();
    do_bindgen(path.join("include"));
}
