use std::io::BufReader;
use std::fs::File;
use anyhow::{ Context, Result };
use log::{info, warn};
use structopt::StructOpt;
extern crate clap_verbosity_flag;

fn answer() -> i32 {
    42
}


#[test]
fn check_answer_validity(){
    assert_eq!(answer(), 42)
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    // flags: -v (warnings) -vv (info) -vvv(debug) and -vvvv (trace)
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern){
            writeln!(writer, "{}", line);
        }
    }  
}

#[test]
fn find_a_match(){
    let mut result: Vec<u8> = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Hello from Print Nanny");
    warn!("Warning from Print Nanny");
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
    
    // find_matches();
    Ok(())
}
