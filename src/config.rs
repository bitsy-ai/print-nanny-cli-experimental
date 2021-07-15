extern crate confy;
use serde::{ Serialize, Deserialize };

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

// #[test]
// fn check_config_missing_api_token(){
//     let config = PrintNannySystemConfig{..PrintNannySystemConfig::default()};
//     let result = check_config(&config);
//     let expected = Err(Error::BlankConfigValue{
//         key: "api_token".to_string()
//     });
//     assert_eq!(result, expected);
// }
// #[test]
// fn check_config_missing_email(){
//     let config = PrintNannySystemConfig{api_token: "abc123".to_string(), ..PrintNannySystemConfig::default()};
//     let result = check_config(&config);
//     let expected = Err(Error::BlankConfigValue{
//         key: "email".to_string()
//     });
//     assert_eq!(result, expected);
// }

pub fn load_config(configfile: &str, default_configfile: &str) -> Result<PrintNannySystemConfig, confy::ConfyError> {
    if configfile == default_configfile {
        return confy::load(configfile); // platform-specific default config path
    } else {
        return confy::load_path(configfile); // load full path instead
    }
}