
use log::{info, warn, error, debug, trace };

use serde::{ Serialize, Deserialize };
use snafu::{ ResultExt, Snafu, ensure };
use anyhow::{ Context, Result };
use futures::executor::block_on;
use dialoguer::Input;

use print_nanny_client::models::{ CallbackTokenAuthRequest, EmailAuthRequest, DetailResponse };
use print_nanny_client::apis::auth_api::{ auth_email_create, auth_token_create, auth_verify_create, AuthEmailCreateError, AuthTokenCreateError, AuthVerifyCreateError };
use print_nanny_client::apis::configuration::{ Configuration as PrintNannyAPIConfig };
use print_nanny_client::apis::Error as PrintNannyClientError;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PrintNannySystemConfig {
    pub api_token: String,
    pub api_url: String,
    pub email: String
}

impl ::std::default::Default for PrintNannySystemConfig {
    fn default() -> Self { Self { 
        api_url: "https://www.print-nanny.com".into(),
        api_token: "".into(),
        email: "".into()
    }}
}

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Received blank config value for key: {} config", key))]
    MissingConfig { key: String },
}

pub fn check_config(config: &PrintNannySystemConfig) ->  Result<(), Error> {
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
    let config = PrintNannySystemConfig{..PrintNannySystemConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::MissingConfig{
        key: "api_token".to_string()
    });
    assert_eq!(result, expected);
}
#[test]
fn check_config_missing_email(){
    let config = PrintNannySystemConfig{api_token: "abc123".to_string(), ..PrintNannySystemConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::MissingConfig{
        key: "email".to_string()
    });
    assert_eq!(result, expected);
}

pub fn load_config(configfile: &str, default_configfile: &str) -> Result<PrintNannySystemConfig, confy::ConfyError> {
    if configfile == default_configfile {
        return confy::load(configfile); // platform-specific default config path
    } else {
        return confy::load_path(configfile); // load full path instead
    }
}

pub fn prompt_token_input(email: &str) -> String {
    let prompt = format!("Please enter the 6-digit code emailed to {}", email.to_string());
    let input : String = Input::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap();
    info!("Received input code {}", input);
    return input;
}

async fn verify_2fa_send_email(email: &str, api_config: &PrintNannyAPIConfig) -> DetailResponse{
    let req =  EmailAuthRequest{email:email.to_string()};
    let res = auth_email_create(&api_config, req).await;

    let result = match res {
        Ok(result) => result,
        Err(e) => panic!("FAILURE in print_nanny_client::apis::auth_email_create  {:?}", e),
    };
    info!("SUCCESS auth_send_verify_email {:?}", serde_json::to_string(&result));
    result
}
pub async fn verify_2fa_auth(email: &str, config: &PrintNannySystemConfig) {
    let api_config = print_nanny_client::apis::configuration::Configuration{
        base_path:config.api_url.to_string(), ..Default::default() 
    };
    verify_2fa_send_email(email, &api_config);
    // info!("SUCCESS auth_send_verify_email {}", serde_json::to_string(auth_res));

}