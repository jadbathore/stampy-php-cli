
fn main(){
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }


    #[cfg(target_os = "windows")]
    {
        exit(code);
        let php_include = std::env::var("PHP_INCLUDE").expect("PHP_INCLUDE not specified");
        println!("cargo:rustc-link-lib=static=ws2_32"); // Bibliothèque Windows pour les sockets
        println!("cargo:include={}", php_include);
        // println!("cargo:rerun-if-changed=build.rs");

        // println!("cargo:rustc-link-search=native={}", php_include);
        // println!("cargo:rustc-link-lib=static=php");
        // println!("cargo:rerun-if-env-changed=PHP_INCLUDE");
        // println!("cargo:rerun-if-env-changed=PHP_LIB_PATH"); 
    }
}