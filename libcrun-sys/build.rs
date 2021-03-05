use std::env;
use std::fs;
use std::io;
use std::io::prelude::Write;
use std::path::PathBuf;
use std::process::Command;

const FUNCTION_ANNONTATION: &str = "LIBCRUN_PUBLIC";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    build_crun()?;
    create_wrapper_h()?;
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:cargo:rustc-link-search=crun/.libs");

    #[cfg(feature = "static")]
    println!("cargo:rustc-link-lib=static=crun");

    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=crun");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");

    generate_bindings();
    Ok(())
}

fn build_crun() -> io::Result<()> {
    env::set_current_dir("crun/")?;
    Command::new("./autogen.sh").output()?;
    Command::new("./configure")
        .arg("--enable-shared")
        .output()?;
    Command::new("make").output()?;
    env::set_current_dir("../")?;
    Ok(())
}

fn create_wrapper_h() -> io::Result<()> {
    let mut wrapper_h = fs::File::create("src/wrapper.h")?;
    let paths = fs::read_dir("crun/src/libcrun/")?;

    for path in paths {
        let filename = path?.path();

        if let Some(ext) = filename.extension() {
            if ext != "h" {
                continue;
            }

            let include_str = format!(
                "#include <libcrun/{}>\n",
                filename
                    .strip_prefix("crun/src/libcrun")
                    .ok()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );

            let contents = fs::read_to_string(&filename)?;
            if contents.contains(FUNCTION_ANNONTATION) {
                wrapper_h.write_all(include_str.as_bytes())?;
            }
        }
    }

    wrapper_h.sync_all()?;

    Ok(())
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-Icrun/src/")
        .clang_arg("-Icrun/libocispec/src/")
        .clang_arg("-Icrun/")
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't find pregenerated bindings!");
}
