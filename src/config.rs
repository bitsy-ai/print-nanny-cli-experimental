
use serde::{ Serialize, Deserialize };
use snafu::{ ResultExt, Snafu, ensure };
use anyhow::{ Context, Result };
use futures::executor::block_on;

use print_nanny_client::models::email_auth_request::{ EmailAuthRequest };
use print_nanny_client::models::email_auth::{ EmailAuth };
use print_nanny_client::apis::auth_api::{ auth_email_create, auth_verify_create, AuthEmailCreateError };
use print_nanny_client::apis::Error as PrintNannyClientError;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PrintNannyConfig {
    api_token: String,
    api_url: String,
    email: String
}

impl ::std::default::Default for PrintNannyConfig {
    fn default() -> Self { Self { 
        api_url: "https://www.print-nanny.com/".into(),
        api_token: "".into(),
        email: "".into()
    }}
}

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Received blank config value for key: {} config", key))]
    MissingConfig { key: String },
}

pub fn check_config(config: &PrintNannyConfig) ->  Result<(), Error> {
    ensure!(!config.api_token.is_empty(), MissingConfig {
        key: "api_token".to_string()
    });
    ensure!(!config.email.is_empty(), MissingConfig {
        key: "email".to_string()
    });
    Ok(())
}
#[test]
fn check_config_missing_api_token(){
    let config = PrintNannyConfig{..PrintNannyConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::MissingConfig{
        key: "api_token".to_string()
    });
    assert_eq!(result, expected);
}
#[test]
fn check_config_missing_email(){
    let config = PrintNannyConfig{api_token: "abc123".to_string(), ..PrintNannyConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::MissingConfig{
        key: "email".to_string()
    });
    assert_eq!(result, expected);
}

pub fn load_config(configfile: &str, default_configfile: &str) -> Result<PrintNannyConfig, confy::ConfyError> {
    if configfile == default_configfile {
        return confy::load(configfile); // platform-specific default config path
    } else {
        return confy::load_path(configfile); // load full path instead
    }
}

pub fn auth_send_verify_email(email: &str, config: &PrintNannyConfig) -> Result<EmailAuth, PrintNannyClientError<AuthEmailCreateError>>  {
    let request = EmailAuthRequest{email:email.to_string()};
    let api_config = print_nanny_client::apis::configuration::Configuration{
        base_path:config.api_url.to_string(), ..Default::default() 
    };
    let future = auth_email_create(&api_config, request);
    block_on(future)
}