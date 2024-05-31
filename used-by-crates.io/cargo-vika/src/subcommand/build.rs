use clap::ArgMatches;
use std::process::Command;

pub fn build(matches: &ArgMatches) {
    let mut build_args = vec!["build"];

    build_args.append(&mut vec!["--package", crate::metadata::metadata().packages.last().unwrap().name.as_str()]);

    build_args.append(&mut vec!["--target", crate::arch::arch()]);

    match matches.value_of("features") {
        None => (),
        Some(features) => {
            build_args.push("--features");
            build_args.append(&mut features.split(",").collect::<Vec<&str>>())
        }
    }

    println!();

    if !Command::new("cargo")
        .args(&build_args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap().success() {
        crate::pretty_output::error("could not build package")
    }
}
