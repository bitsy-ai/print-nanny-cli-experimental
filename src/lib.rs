
use serde::{Serialize, Deserialize};
use snafu::{ResultExt, Snafu, ensure };
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PrintNannyConfig {
    api_key: String,
    api_url: String,
    email: String
}

impl ::std::default::Default for PrintNannyConfig {
    fn default() -> Self { Self { 
        api_url: "https://www.print-nanny.com/api/".into(),
        api_key: "".into(),
        email: "".into()
    }}
}

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Received blank config value for key: {} config", key))]
    AuthRequired { key: String },
}

pub fn check_config(config: &PrintNannyConfig) ->  Result<(), Error> {
    ensure!(!config.api_key.is_empty(), AuthRequired {
        key: "api_key".to_string()
    });
    ensure!(!config.email.is_empty(), AuthRequired {
        key: "email".to_string()
    });
    Ok(())
}

pub fn load_config(configfile: &str, default_configfile: &str) -> Result<PrintNannyConfig, confy::ConfyError> {
    if configfile == default_configfile {
        return confy::load(configfile); // platform-specific default config path
    } else {
        return confy::load_path(configfile); // load full path instead
    }
}

#[test]
fn check_config_missing_api_key(){
    let config = PrintNannyConfig{..PrintNannyConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::AuthRequired{
        key: "api_key".to_string()
    });
    assert_eq!(result, expected);
}
#[test]
fn check_config_missing_email(){
    let config = PrintNannyConfig{api_key: "abc123".to_string(), ..PrintNannyConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::AuthRequired{
        key: "email".to_string()
    });
    assert_eq!(result, expected);
}