use clap::{Arg, App, SubCommand};

extern crate cargo_vika;

fn main() {
    match cargo_metadata::MetadataCommand::new().exec() {
        Ok(meta) => cargo_vika::metadata::init(meta),
        // we can just ignore unsuccessful access to metadata,
        // because it means we are not in package
        // and only commands such `new` can be executed,
        // but metadata is controlled by themselves
        Err(_) => ()
    }

    let matches = App::new("cargo-vika")
        // creates new packages, just like `cargo new`,
        // but puts UEFI template inside it
        .subcommand(SubCommand::with_name("new")
            .arg(Arg::with_name("name")
                .required(true)))
        // builds & runs package
        .subcommand(SubCommand::with_name("run")
            // runs package in QEMU emulator
            .subcommand(SubCommand::with_name("qemu")
                // path to OVMF files,
                // if nothing is specified,
                // it firstly is trying to find files in the current package,
                // and then in system paths
                .arg(Arg::with_name("ovmf")
                    .long("ovmf")
                    .takes_value(true))
                // port that is used to exit QEMU,
                // default 0xF4
                .arg(Arg::with_name("exit_port")
                    .long("exitp")
                    .takes_value(true))
                // number of cores to emulate,
                // default 4
                .arg(Arg::with_name("cores")
                    .long("cores")
                    .takes_value(true))
                // amount of memory to emulate,
                // default 256M
                .arg(Arg::with_name("mem")
                    .long("mem")
                    .takes_value(true))
                // VGA device to be used by qemu(- to disable),
                // default std
                .arg(Arg::with_name("vga")
                    .long("vga")
                    .takes_value(true))
                // specify extra devices to use by qemu
                .arg(Arg::with_name("device")
                    .long("dev")
                    .takes_value(true)
                    .multiple(true))
                // specify features tho be used by crate,
                // they have to be separated by comma,
                // e.g. to enable features `foo`, `bar` and `baz`
                // you need to specify `--features=foo,bar,baz`
                .arg(Arg::with_name("features")
                    .long("features")
                    .takes_value(true))
                // debug the program or not
                .arg(Arg::with_name("debug")
                    .long("debug"))
                // specify which tcp port to use
                .arg(Arg::with_name("debug-port")
                    .requires("debug")
                    .long("debug-port"))
                // if set, everything app prints will be printed to console too
                .arg(Arg::with_name("pass-output")
                    .long("pass-output"))))
        // builds package
        .subcommand(SubCommand::with_name("build")
            // same as in `run` command
            .arg(Arg::with_name("features")
                .long("features")
                .takes_value(true)))
    .get_matches_from(std::env::args().skip(1));

    cargo_vika::subcommands!(matches, subcommands {
        new,
        run,
        build
    })
}
