mod new;
mod run;
mod build;

pub use new::*;
pub use run::*;
pub use build::*;

#[macro_export]
macro_rules! subcommands {
    ($matches:expr, subcommands { $first:ident $(, $other:ident)* $(,)? }) => {
        if let Some(m) = $matches.subcommand_matches(stringify!($first)) {
            $crate::subcommand::$first(m)
        }
        $(else if let Some(m) = $matches.subcommand_matches(stringify!($other)) {
            $crate::subcommand::$other(m)
        })*
        else {
            $crate::pretty_output::error("missing subcommand, try `--help` to see possible ones")
        }
    };
}
