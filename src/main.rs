extern crate clap;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod config;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Log Analyzer")
                          .version("1.0")
                          .author("Jan Schulte <hello@unexpected-co.de>")
                          .about("Analyzes your log files")
                          .subcommand(SubCommand::with_name("config")
                                      .arg(Arg::with_name("validate")
                                          .short("v")
                                          .value_name("FILE")
                                          .takes_value(true)
                                          .required(true)
                                          .help("Validates your config file")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("config") {
        if matches.is_present("validate") {
            println!("Validating config file...");
            let filename = matches.value_of("validate").unwrap();
            match config::read_config_from_file(filename) {
                Ok(_) => println!("config valid"),
                Err(err) => println!("{:?}", err)
            }
        }
    }
}
