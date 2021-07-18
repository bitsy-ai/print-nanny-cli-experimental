extern crate confy;
use serde::{ Serialize, Deserialize };
use thiserror::Error;
use anyhow::{ Result };
use print_nanny_client::models::{ 
    DeviceIdentity
};
#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PrintNannySystemConfig {
    #[serde(default)]
    pub api_url: String,
    #[serde(default)]
    pub api_token: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub device_identity: Option<DeviceIdentity>
}

impl ::std::default::Default for PrintNannySystemConfig {
    fn default() -> Self { Self { 
        api_url: "https://www.print-nanny.com".to_string(),
        api_token: None,
        email: None,
        device_identity: None
    }}
}

// pub fn check_config(config: &PrintNannySystemConfig) ->  Result<(), ConfigError> {
//     if config.api_token.is_none()() {
//        Err(ConfigError::MissingAttribute("api_token".to_string()))
//     } else if config.email.is_empty() {
//         Err(ConfigError::MissingAttribute("email".to_string()))
//     }else {
//         Ok(())
//     }
// }

// #[test]
// fn check_config_missing_api_token(){
//     let config = PrintNannySystemConfig{..PrintNannySystemConfig::default()};
//     let result = check_config(&config);
//     let expected = Err(ConfigError::MissingAttribute("api_token".to_string()));
//     assert_eq!(result, expected);
// }
// #[test]
// fn check_config_missing_email(){
//     let config = PrintNannySystemConfig{api_token: "abc123".to_string(), ..PrintNannySystemConfig::default()};
//     let result = check_config(&config);
//     let expected = Err(ConfigError::MissingAttribute("email".to_string()));
//     assert_eq!(result, expected);
// }

pub fn load_config(app_name: &str, config_name: &str) -> Result<PrintNannySystemConfig, confy::ConfyError> {
    return confy::load(app_name, config_name); // platform-specific default config path
}

pub fn config_show(config: &PrintNannySystemConfig) {
    println!("💜 Your current config:");
    println!("{:#?}", config);
}