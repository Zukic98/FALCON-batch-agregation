use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let mut build = cc::Build::new();

    // Govorimo C kompajleru gdje da traži .h header fajlove
    build.include("c-source/common");
    build.include("c-source/falcon512");

    // 1. Dodajemo sve .c fajlove iz 'common' foldera
    if let Ok(entries) = fs::read_dir("c-source/common") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().unwrap_or_default() == "c" {
                build.file(path);
            }
        }
    }

    // 2. Dodajemo sve .c fajlove iz 'falcon512' foldera
    if let Ok(entries) = fs::read_dir("c-source/falcon512") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().unwrap_or_default() == "c" {
                build.file(path);
            }
        }
    }

    // Optimizacije i kompajliranje
    build.flag("-O3");
    build.compile("falcon_c");

    // 3. Generisanje Rust bindinga (bazirano na glavnom API-ju Falcona)
    let bindings = bindgen::Builder::default()
        .header("c-source/falcon512/api.h") // api.h je glavni fajl u PQClean-u
        // Dodajemo i common headere ako bindgenu zatrebaju
        .clang_arg("-I./c-source/common")
        .clang_arg("-I./c-source/falcon512")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Nije uspjelo generisanje bindinga");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Nije uspjelo snimanje bindings.rs");
}