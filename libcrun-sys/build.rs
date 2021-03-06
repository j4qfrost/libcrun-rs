use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const FUNCTION_ANNONTATION: &str = "LIBCRUN_PUBLIC";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    build_crun()?;
    create_wrapper_h()?;

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");

    generate_bindings()?;
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

            let contents = fs::read_to_string(&filename)?;
            let re = format!(r"{}[^;]*", FUNCTION_ANNONTATION);
            let re_result = Regex::new(&re).unwrap();
            let iter: Vec<_> = re_result.find_iter(&contents).collect();
            if iter.len() > 0 {
                let include_str = format!(
                    "\n#include <libcrun/{}>",
                    filename
                        .strip_prefix("crun/src/libcrun")
                        .ok()
                        .unwrap()
                        .to_str()
                        .unwrap()
                );
                wrapper_h.write_all(include_str.as_bytes())?;
                for mat in iter {
                    let mut line = mat.as_str().to_owned();
                    line.replace_range(..FUNCTION_ANNONTATION.len(), "\n/*");
                    line.push_str("; */\n");
                    wrapper_h.write_all(line.as_bytes())?;
                }
            }
        }
    }

    wrapper_h.sync_all()?;

    Ok(())
}

fn generate_bindings() -> io::Result<()> {
    let lib_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("crun/.libs");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-search=/lib/x86_64-linux-gnu/");
    #[cfg(not(features = "static"))]
    {
        println!("cargo:rustc-link-lib=crun");
        println!("cargo:rustc-link-lib=yajl");
        println!("cargo:rustc-link-lib=systemd");
        println!("cargo:rustc-link-lib=seccomp");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=cap");
    }
    #[cfg(features = "static")]
    {
        println!("cargo:rustc-link-lib=static=crun");
        println!("cargo:rustc-link-lib=static=yajl");
        println!("cargo:rustc-link-lib=static=systemd");
        println!("cargo:rustc-link-lib=static=seccomp");
        println!("cargo:rustc-link-lib=static=dl");
        println!("cargo:rustc-link-lib=static=cap");
    }
    let bindings = bindgen::Builder::default()
        .clang_arg("-Icrun/src/")
        .clang_arg("-Icrun/libocispec/src/")
        .clang_arg("-Icrun/")
        .clang_arg("-I/usr/include/yajl/")
        .clang_arg("-Lcrun/.libs/")
        .clang_arg("-L/lib/x86_64-linux-gnu/")
        .header("src/wrapper.h")
        .trust_clang_mangling(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;
    fs::copy(lib_dir.join("libcrun.so.0"), out_path.join("libcrun.so.0"))?;
    Ok(())
}

// FOUND_LIBS = -lyajl -lsystemd -lseccomp -ldl -lcap
