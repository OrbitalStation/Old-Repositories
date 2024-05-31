use termcolor::{StandardStream, ColorChoice, ColorSpec, Color, WriteColor};
use std::io::Write;

pub fn standard_spec() -> ColorSpec {
    let mut standard_spec = ColorSpec::new();
    standard_spec.set_fg(None);
    standard_spec.set_bold(false);
    standard_spec
}

fn template(msg1: &str, msg2: &str, spec: &ColorSpec) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(&spec).unwrap();
    write!(&mut stdout, "{}", msg1).unwrap();

    stdout.set_color(&standard_spec()).unwrap();
    writeln!(&mut stdout, "{}", msg2).unwrap()
}

pub fn command(msg1: &str, msg2: &str) {
    template(msg1, msg2, ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))
}

pub fn warning(msg: &str) {
    template("     warning", format!(":{}", msg).as_str(), ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
}

pub fn error(msg: &str) -> ! {
    template("       error", format!(": {}", msg).as_str(), ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
    std::process::exit(1)
}
