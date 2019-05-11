extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};

use crate::audio::set_volume;
use std::error::Error;
use simple_error::*;

mod audio;
mod rule;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}

fn run() -> Result<(), Box<Error>> {
    let app = App::new("volumectl")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1.0")
        .subcommand(SubCommand::with_name("set-volume")
            .about("Sets the volume of the configured sink inputs")
            .arg(Arg::with_name("VOLUME")
                .required(true)
                .help("The volume to set. e.g. 0.3")
            )
        );

    let matches = app.get_matches();


    if let Some(matches) = matches.subcommand_matches("set-volume") {
        let volume: f64 = try_with!(matches.value_of("VOLUME").unwrap().parse(), "Could not parse volume");
        if !(volume >= 0.0 && volume <= 1.0) {
            bail!("Volume must be between 0 and 1. Got {} instead.", volume);
        }
        let rules = rule::read_rules()?;

        set_volume(rules, volume)?;
    }

    Ok(())
}