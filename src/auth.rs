
use log::{info };
use anyhow::{ Context, Result };

use print_nanny_client::models::{ 
    CallbackTokenAuthRequest,
    DetailResponse,
    EmailAuthRequest,
    TokenResponse,
};
use print_nanny_client::apis::auth_api::{ auth_email_create, auth_token_create };
use print_nanny_client::apis::configuration::{ Configuration as PrintNannyAPIConfig };
use crate::config::{ PrintNannySystemConfig };
use crate::device::{ device_identity_update_or_create };
use crate::prompt::{ prompt_device_name, prompt_token_input };

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
        .context("🔴 Verification failed. Please try again or contact leigh@print-nanny.com for help.")?;
    info!("SUCCESS auth_verify_create detail {:?}", serde_json::to_string(&res));
    Ok(res)
}

pub async fn verify_2fa_auth(config: &PrintNannySystemConfig) -> Result<TokenResponse> {
    let api_config = print_nanny_client::apis::configuration::Configuration{
        base_path:config.api_url.to_string(), ..Default::default() 
    };
    verify_2fa_send_email(&api_config, &config.email).await?;
    println!("📥 Sent a 6-digit verification code to {}", config.email);

    let otp_token = prompt_token_input(&config.email);
    // let verified = verify_api_token(&api_config, api_token, email).await;
    println!("✅ Success! Your email was verified {}", config.email);
    println!("⏳ Registering your device. Please wait for completion.");
    let api_token = verify_2fa_code(&api_config, otp_token, &config.email).await?;
    Ok(api_token)
}

pub async fn auth(config: &mut PrintNannySystemConfig) -> Result<()> {
    let token_res = verify_2fa_auth(&config).await?;
    config.api_token = token_res.token;
    confy::store("printnanny", config.clone())?;
    let device_name = prompt_device_name();
    let pki_res = device_identity_update_or_create(config, &device_name);

    Ok(())
}