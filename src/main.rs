use std::io::BufReader;
use std::fs::File;
use anyhow::{ Context, Result };
use log::{info, warn, error, debug, trace };
use structopt::StructOpt;
use printnanny::config::{ 
    // check_config, 
    load_config, 
    verify_2fa_auth
};

use env_logger::Builder;
use log::LevelFilter;

extern crate clap;
use clap::{ Arg, App, SubCommand };
extern crate confy;


#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Builder::new();
    
    let matches = App::new("printnanny")
        .version("0.1.0")
        .author("Leigh Johnson <leigh@bitsy.ai>")
        .about("Official Print Nanny CLI https://print-nanny.com")
        .arg(Arg::with_name("api-url")
            .long("api-url")
            .help("Specify api_url")
            .value_name("API_URL")
            .takes_value(true))
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
    let verbosity = matches.occurrences_of("v");
    match verbosity {
        0 => builder.filter_level(LevelFilter::Warn).init(),
        1 => builder.filter_level(LevelFilter::Info).init(),
        2 => builder.filter_level(LevelFilter::Debug).init(),
        3 | _ => builder.filter_level(LevelFilter::Trace).init(),
    };
    
    let mut config = load_config(&configfile, &default_configfile)?;

    // let mut rt = tokio::runtime::Runtime::new().unwrap();

    if let Some(api_url) = matches.value_of("api-url") {
        config.api_url = api_url.to_string();
        info!("Using api-url {}", config.api_url);
    }
    if let Some(matches) = matches.subcommand_matches("auth") {
        let email = matches.value_of("email").unwrap();
        info!("Sending two-factor auth request for {}", email.to_string());
        // let verify_2fa_response = verify_2fa_auth(email, &config).await;

        if let Err(err) = verify_2fa_auth(email, &config).await {
            if verbosity > 0 {
                eprintln!("Error: {:#?}", err);
            } else {
                eprintln!("Error: {:?}", err);    
            }
            std::process::exit(1);
        }
    }

    

    Ok(())
}
