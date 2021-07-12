use std::io::BufReader;
use std::fs::File;
use anyhow::{ Context, Result };
use log::{info, warn, error, debug, trace };
use structopt::StructOpt;
use printnanny::config::{ PrintNannyConfig, check_config, load_config, auth_send_verify_email };
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
            .about("Update Print Nanny configuration"))
        .subcommand(SubCommand::with_name("auth")
            .about("Connect your Print Nanny account")
            .arg(Arg::with_name("email")
                .long("email")
                .help("Enter the email associated with your Print Nanny account")
                .value_name("EMAIL")
                .takes_value(true)
                .required(true)
            ))
        .get_matches();


    let default_configfile = "printnanny";
    let configfile = matches.value_of("config").unwrap_or(default_configfile);
    info!("Using config file: {}", configfile);

    let config = load_config(&configfile, &default_configfile)?;
    if let Some(matches) = matches.subcommand_matches("auth") {
        let email = matches.value_of("email").unwrap();
        auth_send_verify_email(email, &config);
    }


    

    Ok(())
}
