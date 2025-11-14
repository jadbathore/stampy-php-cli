use std::error::Error;

fn main()->Result<(),Box<dyn Error>> 
{

    // #[cfg(target_os = "macos")]
    // {
    //     let test = cfg!(target_os = "windows");
    //     dbg!(test);
    //     // println!("windows");
    //     let php_include = std::env::var("PHP_INCLUDE").expect("PHP_INCLUDE not specified");
    //     println!("cargo:rustc-link-lib=static=ws2_32");
    //     println!("cargo:include={}", php_include);
    //     // println!("cargo:rerun-if-changed=build.rs");

    //     // println!("cargo:rustc-link-search=native={}", php_include);
    //     // println!("cargo:rustc-link-lib=static=php");
    //     // println!("cargo:rerun-if-env-changed=PHP_INCLUDE");
    //     // println!("cargo:rerun-if-env-changed=PHP_LIB_PATH"); 
    // }

    // let tty_path = std::fs::read_link("/proc/self/fd/0")?;
    // dbg!(tty_path);
    Ok(())
}