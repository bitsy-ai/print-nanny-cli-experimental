use std::io::BufReader;
use std::fs::File;
use anyhow::{ Context, Result };
use log::{info, warn, error, debug, trace };
use structopt::StructOpt;
use printnanny::{ PrintNannyConfig };
extern crate clap;
use clap::{ Arg, App, SubCommand };
extern crate confy;


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


    let default_configfile = "printnanny";
    let configfile = matches.value_of("config").unwrap_or(default_configfile);
    info!("Using config file: {}", configfile);

    if configfile == default_configfile {
        let config = confy::load(configfile)?; // platform-specific default config path
    } else {
        let config = confy::load_path(configfile)?; // load full path instead
    }

    if let Some(matches) = matches.subcommand_matches("configure") {
        // printnanny::configure_prompts()
    }

    Ok(())
}
