use std::io::BufReader;
use std::fs::File;
use anyhow::{ Context, Result };
use log::{info, warn, error, debug, trace };
use structopt::StructOpt;
extern crate clap_verbosity_flag;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    // flags: -v (warnings) -vv (info) -vvv(debug) and -vvvv (trace)
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity
}

fn main() -> Result<()> {
    env_logger::init();

    trace!("Example TRACE from Print Nanny");
    debug!("Example DEBUG from Print Nanny");
    info!("Example INFO from Print Nanny");
    warn!("Example WARNING from Print Nanny");
    error!("Example ERROR from Print Nanny");
    let args = Cli::from_args();
    
    // TODO read from BufReader instead of loading entire file into memory
    // let f = File::open(&args.path)?;
    // let mut reader = BufReader::new(f);
    // let mut line = String::new();

    // for line in reader.lines() {
    //     if line.contains(&args.pattern){
    //         println!("{}", line);
    //     }  
    // }
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    let mut result: Vec<u8> = Vec::new();

    printnanny::find_matches(&content, &args.pattern, result);
    Ok(())
}
