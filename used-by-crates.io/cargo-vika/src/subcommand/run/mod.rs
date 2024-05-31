mod qemu;

use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("qemu") {
        super::build(matches);
        qemu::qemu(matches)
    } else {
        crate::pretty_output::error("missing subcommand for `run`, possible values: (qemu/real)")
    }
}
