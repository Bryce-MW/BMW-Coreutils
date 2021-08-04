use std::{
    env,
    path::Path,
    process::Command,
    fs
};
use std::error::Error;
use cargo_clone;
use cargo::core::SourceId;
use cargo::util::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let bmwc_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let profile = env::var("PROFILE").unwrap();

    let mut cargo_config = Config::default().expect("Unable to get config.");
    cargo_config.configure(
        0,
        Some(true),
        &None,
        false,
        false,
        false,
        &None,
        &[],
    )?;
    let source_id = SourceId::crates_io(&cargo_config).unwrap();

    println!("cargo:rustc-link-search={}/bmwc/bin", out_dir);
    fs::create_dir_all(format!("{}/bmwc/bin", out_dir))?;

    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=resolv");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=System");

    if Path::new(&format!("{}/libc", &bmwc_dir)).exists() {
        cargo_build(&bmwc_dir, "libc")?;
        fs::copy(format!("{}/libc/target/{}/libc.a", &bmwc_dir, profile), format!("{}/bmwc/bin/libc.a", &out_dir))?;
    } else {
        let bmwlibc_dir = format!("{}/bmwc/bmwlibc", out_dir);
        cargo_clone::ops::clone(Some("bmwlibc"), &source_id, Some(&bmwlibc_dir), None, &cargo_config)?;
        cargo_build(&bmwlibc_dir, "")?;
        fs::copy(format!("{}/target/{}/libc.a", &bmwlibc_dir, profile), format!("{}/bmwc/bin/libc.a", &out_dir))?;
    }
    if Path::new(&format!("{}/libm", &bmwc_dir)).exists() {
        cargo_build(&bmwc_dir, "libm")?;
        fs::copy(format!("{}/libm/target/{}/libm.a", &bmwc_dir, profile), format!("{}/bmwc/bin/libm.a", &out_dir))?;
    } else {
        let bmwlibm_dir = format!("{}/bmwc/bmwlibm", out_dir);
        cargo_clone::ops::clone(Some("bmwlibm"), &source_id, Some(&bmwlibm_dir), None, &cargo_config)?;
        cargo_build(&bmwlibm_dir, "")?;
        fs::copy(format!("{}/target/{}/libm.a", &bmwlibm_dir, profile), format!("{}/bmwc/bin/libm.a", &out_dir))?;
    }
    if Path::new(&format!("{}/libresolv", &bmwc_dir)).exists() {
        cargo_build(&bmwc_dir, "libresolv")?;
        fs::copy(format!("{}/libresolv/target/{}/libresolv.a", &bmwc_dir, profile), format!("{}/bmwc/bin/libresolv.a", &out_dir))?;
    } else {
        let bmwlibresolv_dir = format!("{}/bmwc/bmwlibresolv", out_dir);
        cargo_clone::ops::clone(Some("bmwlibresolv"), &source_id, Some(&bmwlibresolv_dir), None, &cargo_config)?;
        cargo_build(&bmwlibresolv_dir, "")?;
        fs::copy(format!("{}/target/{}/libresolv.a", &bmwlibresolv_dir, profile), format!("{}/bmwc/bin/libresolv.a", &out_dir))?;
    }
    if Path::new(&format!("{}/libiconv", &bmwc_dir)).exists() {
        cargo_build(&bmwc_dir, "libiconv")?;
        fs::copy(format!("{}/libiconv/target/{}/libiconv.a", &bmwc_dir, profile), format!("{}/bmwc/bin/libiconv.a", &out_dir))?;
    } else {
        let bmwlibiconv_dir = format!("{}/bmwc/bmwlibiconv", out_dir);
        cargo_clone::ops::clone(Some("bmwlibiconv"), &source_id, Some(&bmwlibiconv_dir), None, &cargo_config)?;
        cargo_build(&bmwlibiconv_dir, "")?;
        fs::copy(format!("{}/target/{}/libiconv.a", &bmwlibiconv_dir, profile), format!("{}/bmwc/bin/libiconv.a", &out_dir))?;
    }
    if Path::new(&format!("{}/libSystem", &bmwc_dir)).exists() {
        cargo_build(&bmwc_dir, "libSystem")?;
        fs::copy(format!("{}/libSystem/target/{}/libSystem.a", &bmwc_dir, profile), format!("{}/bmwc/bin/libSystem.a", &out_dir))?;
    } else {
        let bmwlibSystem_dir = format!("{}/bmwc/bmwlibSystem", out_dir);
        cargo_clone::ops::clone(Some("bmwlibSystem"), &source_id, Some(&bmwlibSystem_dir), None, &cargo_config)?;
        cargo_build(&bmwlibSystem_dir, "")?;
        fs::copy(format!("{}/target/{}/libSystem.a", &bmwlibSystem_dir, profile), format!("{}/bmwc/bin/libSystem.a", &out_dir))?;
    }

    eprintln!("{:?}", Command::new("exa")
        .arg("--tree")
        .current_dir(out_dir)
        .output()
        .unwrap());

    Ok(())
}

fn cargo_build(dir: &str, subdir: &str) -> Result<(), Box<dyn Error>> {
    Command::new("cargo")
        .current_dir(format!("{}/{}", dir, subdir))
        .arg("build")
        .spawn()
        .unwrap()
        .wait()?;

    Ok(())
}
