use std::env;
use std::path::PathBuf;

use bindgen;
use cmake::Config;

static HEADER: &str = "external/openmmapi/include/OpenMM.h";

fn cmake_and_build() -> PathBuf {
    Config::new("external")
        .define("OPENMM_BUILD_PYTHON_WRAPPERS", "OFF")
        .define("OPENMM_BUILD_STATIC_LIB", "OFF")
        .define("OPENMM_BUILD_SHARED_LIB", "ON")
        .build()
}

fn link(build_path: PathBuf) {
    println!("cargo:rustc-link-search=native={}", build_path.display());
    println!("cargo:rustc-link-lib=dylib=OpenMM");
    // println!("cargo:rustc-link-lib=static=OpenMM_static");
}

fn do_bindgen() {
    println!("cargo:rerun-if-changed={}", HEADER);

    let bindings = bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=gnu++11")
        .clang_arg("-Iexternal/libraries/asmjit")
        .clang_arg("-Iexternal/libraries/vecmath/include")
        .clang_arg("-Iexternal/libraries/irrxml/include")
        .clang_arg("-Iexternal/serialization/include")
        .clang_arg("-Iexternal/platforms/reference/include")
        .clang_arg("-Iexternal/libraries/csha1/include")
        .clang_arg("-Iexternal/libraries/hilbert/include")
        .clang_arg("-Iexternal/libraries/lbfgs/include")
        .clang_arg("-Iexternal/libraries/sfmt/include")
        .clang_arg("-Iexternal/libraries/lepton/include")
        .clang_arg("-Iexternal/libraries/quern/include")
        .clang_arg("-Iexternal/libraries/jama/include")
        .clang_arg("-Iexternal/olla/include")
        .clang_arg("-Iexternal/openmmapi/include")
        .clang_arg("-Iexternal/tests")
        .enable_cxx_namespaces()
        .rustfmt_bindings(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .array_pointers_in_arguments(true)
        .whitelist_function("OpenMM::.*")
        .whitelist_var("OpenMM::.*")
        .whitelist_type("OpenMM::.*")
        .header(HEADER)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let build_path = cmake_and_build();

    do_bindgen();

    link(build_path);
}
