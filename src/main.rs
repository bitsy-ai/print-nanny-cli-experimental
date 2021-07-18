use anyhow::{ Result };
use log::{info };
use printnanny::config::{ 
    load_config,
    config_show
};
use env_logger::Builder;
use log::LevelFilter;
extern crate clap;
use clap::{ Arg, App, SubCommand };
use clap::{ AppSettings };
extern crate confy;
use printnanny::auth::{ auth };
use printnanny::printer::{ printer_add };

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Builder::new();
    
    let app = App::new("printnanny")
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
        .subcommand(SubCommand::with_name("auth")
            .about("Connect your Print Nanny account")
            .arg(Arg::with_name("email")
                .long("email")
                .help("Enter the email associated with your Print Nanny account")
                .value_name("EMAIL")
                .takes_value(true)
                .required(true)
            ))
        .subcommand(SubCommand::with_name("config")
            .about("Manage Print Nanny configuration")
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("show")
            .about("Show current Print Nanny configuration")))
        .subcommand(SubCommand::with_name("camera")
            .about("Manage camera monitored by Print Nanny")
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("add")
                .about("Add a camera to Print Nanny"))
            .subcommand(SubCommand::with_name("remove"))
                .about("Remove a camera")
        );
        
    let app_m = app.get_matches();

    let default_configfile = "printnanny";
    let configfile = app_m.value_of("config").unwrap_or(default_configfile);
    info!("Using config file: {}", configfile);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'printnanny -v -v -v' or 'printnanny -vvv' vs 'printnanny -v'
    let verbosity = app_m.occurrences_of("v");
    match verbosity {
        0 => builder.filter_level(LevelFilter::Warn).init(),
        1 => builder.filter_level(LevelFilter::Info).init(),
        2 => builder.filter_level(LevelFilter::Debug).init(),
        3 | _ => builder.filter_level(LevelFilter::Trace).init(),
    };
    
    let mut config = load_config(&configfile, &default_configfile)?;
    if let Some(api_url) = app_m.value_of("api-url") {
        config.api_url = api_url.to_string();
        info!("Using api-url {}", config.api_url);
    }

    match app_m.subcommand() {
        ("auth", Some(_sub_m)) => {
            if let Some(email) = app_m.value_of("email"){
                config.email = email.to_string();
            }
            if let Err(err) = auth(&mut config).await {
                if verbosity > 0 {
                    eprintln!("Error: {:#?}", err);
                } else {
                    eprintln!("Error: {:?}", err);    
                }
                std::process::exit(1);
            };
        },
        ("config", Some(sub_m)) => {
            match sub_m.subcommand() {
                ("show", Some(_config_m)) => config_show(&config),
                _ => {}
            }
        },
        ("printer", Some(sub_m)) => {
            match sub_m.subcommand() {
                ("add", Some(_printer_m)) => printer_add(&mut config).await?,
                _ => {}
            }
        },
        _ => {}
    }
    Ok(())
}
