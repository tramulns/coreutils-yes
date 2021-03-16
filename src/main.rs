use clap::{App, Arg};
use std::borrow::Cow;
use std::io::{self, Write};

fn main() {
    let app = App::new("yes").arg(Arg::with_name("STRING").index(1).multiple(true));

    let matches = match app.get_matches_from_safe(std::env::args_os().into_iter()) {
        Ok(m) => m,
        Err(ref e)
            if e.kind == clap::ErrorKind::HelpDisplayed
                || e.kind == clap::ErrorKind::VersionDisplayed =>
        {
            println!("{}", e);
            std::process::exit(0);
        }
        Err(f) => {
            eprintln!("{}", f);
            std::process::exit(1)
        }
    };

    let string: Cow<str> = if let Some(values) = matches.values_of("STRING") {
        let mut result = values.fold(String::new(), |res, s| format!("{}{} ", res, s));
        result.pop();
        result.push('\n');
        result.into()
    } else {
        "y\n".into()
    };

    let bytes = string.as_bytes();
    exec(bytes).unwrap();
}

fn exec(buf: &[u8]) -> std::io::Result<()> {
    loop {
        io::stdout().write_all(buf)?;
    }
}
