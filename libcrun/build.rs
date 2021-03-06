use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let lib_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../libcrun-sys/crun/.libs")
        .canonicalize()?;
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_path.display());
    println!("cargo:rustc-link-search=/lib/x86_64-linux-gnu/");
    #[cfg(not(features = "static"))]
    {
        fs::copy(lib_dir.join("libcrun.so.0"), out_path.join("libcrun.so.0"))?;
        println!("cargo:rustc-link-lib=crun");
        println!("cargo:rustc-link-lib=yajl");
        println!("cargo:rustc-link-lib=systemd");
        println!("cargo:rustc-link-lib=seccomp");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=cap");
    }
    #[cfg(features = "static")]
    {
        fs::copy(lib_dir.join("libcrun.a"), out_path.join("libcrun.a"))?;
        println!("cargo:rustc-link-lib=static=crun");
        println!("cargo:rustc-link-lib=static=yajl");
        println!("cargo:rustc-link-lib=static=systemd");
        println!("cargo:rustc-link-lib=static=seccomp");
        println!("cargo:rustc-link-lib=static=dl");
        println!("cargo:rustc-link-lib=static=cap");
    }

    Ok(())
}
