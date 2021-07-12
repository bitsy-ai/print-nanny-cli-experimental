use std::io::BufReader;
use std::fs::File;
use anyhow::{ Context, Result };
use log::{info, warn, error, debug, trace };
use structopt::StructOpt;
use serde::{Serialize, Deserialize};
extern crate clap;
use clap::{ Arg, App, SubCommand };
extern crate confy;


#[derive(Serialize, Deserialize)]
struct PrintNannyConfig {
    api_key: String
}

impl ::std::default::Default for PrintNannyConfig {
    fn default() -> Self { Self { api_key: "".into() }}
}

fn configure_prompts(){}

fn main() -> Result<()> {
    env_logger::init();

    let matches = App::new("printnanny")
        .version("0.1.0")
        .author("Leigh Johnson <leigh@bitsy.ai>")
        .about("Official Print Nanny CLI https://print-nanny.com")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("Load custom config file")
            .value_name("FILE")
            .takes_value(true))
        .subcommand(SubCommand::with_name("configure")
            .about("Configure your Print Nanny account"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    info!("Using config file: {}", config);

    if let Some(matches) = matches.subcommand_matches("configure") {
        configure_prompts()
    }

    Ok(())
}
