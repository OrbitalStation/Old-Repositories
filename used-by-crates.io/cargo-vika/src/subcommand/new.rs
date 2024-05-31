use clap::ArgMatches;
use std::{
    path::Path,
    fs
};

pub fn new(matches: &ArgMatches) {
    let package_name = matches.value_of("name").unwrap();
    let package = Path::new(package_name);
    let cargo_toml = package.join("Cargo.toml");
    let src = package.join("src");
    let main_rs = src.join("main.rs");
    let _cargo = package.join(".cargo");
    let config = _cargo.join("config");
    let _gitignore = package.join(".gitignore");

    fs::create_dir(package).unwrap();
    fs::write(cargo_toml, format!(
        "[package]\n\
        name = \"{}\"\n\
        version = \"0.0.0\"\n\
        edition = \"2021\"\n\
        \n\
        [dependencies]\n\
        uefi = \"0.12.*\"\n\
        uefi-services = \"0.9.*\"\n",
    package_name)).unwrap();
    fs::create_dir(src).unwrap();
    fs::write(main_rs, "\
        #![no_main]\n\
        #![no_std]\n\
        #![feature(abi_efiapi)]\n\
        \n\
        use uefi::prelude::*;\n\
        use uefi::ResultExt;\n\
        \n\
        #[entry]\n\
        fn main(_handle: Handle, mut st: SystemTable <Boot>) -> Status {\n\
            \tuefi_services::init(&mut st).unwrap_success();\n\
            \n\
            \tStatus::SUCCESS\n\
        }\n"
    ).unwrap();
    fs::create_dir(_cargo).unwrap();
    fs::write(config, "\
        [unstable]\n\
        build-std = [\"core\", \"compiler_builtins\", \"alloc\"]\n\
        build-std-features = [\"compiler-builtins-mem\"]\n"
    ).unwrap();
    fs::write(_gitignore, "\
        .idea\n\
        target\n\
        \n\
        Cargo.lock\n"
    ).unwrap();

    crate::pretty_output::command("     Created ", format!("binary (application) `{}` package", package_name).as_str())
}
