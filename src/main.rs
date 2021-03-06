// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#![cfg_attr(feature = "fail-on-deprecated", deny(deprecated))]

use cli::Config;
use error::*;
use info::{repo, Info};
use std::{io, process};
use ui::printer::Printer;

mod cli;
mod error;
mod info;
mod ui;

fn run() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let config = Config::new()?;

    if config.print_languages {
        return cli::print_supported_languages();
    }

    if config.print_package_managers {
        return cli::print_supported_package_managers();
    }

    if !repo::is_valid(&config.repo_path)? {
        return Err("please run onefetch inside of a non-bare git repository".into());
    }

    let info = Info::new(config)?;

    let mut printer = Printer::new(io::BufWriter::new(io::stdout()), info);

    printer.print()?;

    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(error) => {
            let stderr = io::stderr();
            default_error_handler(&error, &mut stderr.lock());
            process::exit(1);
        }
    }
}
