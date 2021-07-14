
use log::{info, warn, error, debug, trace };

use serde::{ Serialize, Deserialize };
use futures::executor::block_on;
use dialoguer::Input;
use anyhow::{ Context, Result };

use print_nanny_client::models::{ 
    CallbackTokenAuthRequest,
    DetailResponse,
    EmailAuthRequest, 
    OctoPrintDevice
    OctoPrintDeviceRequest,
    TokenResponse,
};
use print_nanny_client::apis::auth_api::{ auth_email_create, auth_token_create, auth_verify_create };
use print_nanny_client::apis::configuration::{ Configuration as PrintNannyAPIConfig };
// use print_nanny_client::apis::{ Error as PrintNannyClientError, ResponseContent };


// https://github.com/shepmaster/snafu/issues/199

// #[derive(Debug, Snafu, PartialEq)]
// pub enum Error {
//     #[snafu(display("🔴 Received blank config value for key: {} config", key))]
//     BlankConfigValue { key: String },
//     #[snafu(display("🔴 Failed to send verification email to {} {:?}", email, source))]
//     PrintNannyAPIError { 
//         email: String, 
//         source: print_nanny_client::apis::Error
//     },
// }

// type Result<T, E = Error> = std::result::Result<T, E>;


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

// pub fn check_config(config: &PrintNannySystemConfig) ->  Result<()> {
//     ensure!(!config.api_token.is_empty(), Error::BlankConfigValue {
//         key: "api_token".to_string()
//     });
//     ensure!(!config.email.is_empty(), Error::BlankConfigValue {
//         key: "email".to_string()
//     });
//     Ok(())
// }

#[test]
fn check_config_missing_api_token(){
    let config = PrintNannySystemConfig{..PrintNannySystemConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::BlankConfigValue{
        key: "api_token".to_string()
    });
    assert_eq!(result, expected);
}
#[test]
fn check_config_missing_email(){
    let config = PrintNannySystemConfig{api_token: "abc123".to_string(), ..PrintNannySystemConfig::default()};
    let result = check_config(&config);
    let expected = Err(Error::BlankConfigValue{
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
    let prompt = format!("⚪ Please enter the 6-digit code emailed to {}", email.to_string());
    let input : String = Input::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap();
    info!("Received input code {}", input);
    return input;
}


async fn verify_2fa_send_email(api_config: &PrintNannyAPIConfig, email: &str) -> Result<DetailResponse> {
    // Sends an email containing an expiring one-time password (6 digits)
    let req =  EmailAuthRequest{email:email.to_string()};
    let res = auth_email_create(&api_config, req).await
        .context(format!("🔴 Failed to send verification email to {}", email))?;
    info!("SUCCESS auth_email_create detail {:?}", serde_json::to_string(&res));
    Ok(res)
}

async fn verify_2fa_code(api_config: &PrintNannyAPIConfig, token: String, email: &str) -> Result<TokenResponse> {
    // Verifies email and one-time password (6 digit pair), returning a Bearer token if verification succeeds
    let req = CallbackTokenAuthRequest{mobile: None, token:token, email:Some(email.to_string())};
    let res = auth_token_create(&api_config, req).await
        .context(format!("🔴 Verification failed. Please try again or contact leigh@print-nanny.com for help."))?;
    info!("SUCCESS auth_verify_create detail {:?}", serde_json::to_string(&res));
    Ok(res)
}

async fn register_device(api_config: &PrintNannyAPIConfig) -> Result<OctoPrintDevice> {

    let cpuinfo = cpuid::identify()?;

    let req = OctoPrintDeviceRequest{};
}

pub async fn verify_2fa_auth(email: &str, config: &PrintNannySystemConfig) -> Result<()> {
    let mut api_config = print_nanny_client::apis::configuration::Configuration{
        base_path:config.api_url.to_string(), ..Default::default() 
    };
    verify_2fa_send_email(&api_config, email).await?;
    println!("📥 Sent a 6-digit verification code to {}", email.to_string());

    let otp_token = prompt_token_input(email);
    // let verified = verify_api_token(&api_config, api_token, email).await;
    println!("✅ Success! Your email was verified {}", email.to_string());
    println!("⏳ Registering your device. Please wait for completion.");
    let api_token = verify_2fa_code(&api_config, otp_token, email).await?;

    Ok(())
}