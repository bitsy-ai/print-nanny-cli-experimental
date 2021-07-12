use std::io::BufReader;
use std::fs::File;
use anyhow::{ Context, Result };
use log::{info, warn, error, debug, trace };
use structopt::StructOpt;
use printnanny::config::{ PrintNannyConfig, check_config, load_config, auth_send_verify_email };
use env_logger::Builder;
use log::LevelFilter;

extern crate clap;
use clap::{ Arg, App, SubCommand };
extern crate confy;


fn main() -> Result<()> {
    let mut builder = Builder::new();
    
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
        .arg(Arg::with_name("v")
        .short("v")
        .multiple(true)
        .help("Sets the level of verbosity"))
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

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'printnanny -v -v -v' or 'printnanny -vvv' vs 'printnanny -v'
    match matches.occurrences_of("v") {
        0 => builder.filter_level(LevelFilter::Warn).init(),
        1 => builder.filter_level(LevelFilter::Info).init(),
        2 => builder.filter_level(LevelFilter::Debug).init(),
        3 | _ => builder.filter_level(LevelFilter::Trace).init(),
    }

    let config = load_config(&configfile, &default_configfile)?;
    if let Some(matches) = matches.subcommand_matches("auth") {
        let email = matches.value_of("email").unwrap();
        info!("Sending two-factor auth request for {}", email.to_string());
        let verify_email_future = auth_send_verify_email(email, &config);

    }

    

    Ok(())
}
