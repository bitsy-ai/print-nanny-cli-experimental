
use log::{ info };
use sysinfo::{ SystemExt };
use thiserror::Error;
use anyhow::{ Context, Result };
use dialoguer::{ Input, MultiSelect, Select };
use print_nanny_client::models::{ 
    PrinterProfileRequest,
    CameraSourceTypeEnum,
    CameraTypeEnum
};

// https://github.com/dtolnay/thiserror
#[derive(Error, Debug)]
pub enum PromptError {
    #[error("🔴 Please enter required field: {0}")]
    Required(String),
}

// TODO use Result<String> instead of String type here
pub fn prompt_email() -> String {
    let prompt = "⚪ Enter your email address";
    Input::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

pub fn prompt_token_input(email: &str) -> String {
    let prompt = format!("⚪ Enter the 6-digit code emailed to {}", email.to_string());
    let input : String = Input::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap();
    info!("Received input code {}", input);
    return input;
}

pub fn prompt_device_name() -> String {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    let hostname = system.host_name().unwrap();
    let prompt = "⚪ Enter a name for this device:";
    let input = Input::new()
        .with_prompt(prompt)
        .default(hostname)
        .interact_text()
        .unwrap();
    input
}

pub fn prompt_camera_type() -> Result<CameraTypeEnum> {
    let prompt = "⚪ Select the type of camera to configure";
    let items = vec![
            CameraTypeEnum::RaspberryPiCameraModule,
            CameraTypeEnum::RaspberryPiUSBCamera,
            CameraTypeEnum::GenericRTSPRTMPIPCamera
    ];
    let idx = Select::new()
        .with_prompt(prompt.to_string())
        .items(&items)
        .default(0)
        .interact()?;
    Ok(items[idx])
}

// TODO create mjpg streamer config 
pub fn prompt_camera_source_type() -> Result<CameraSourceTypeEnum> {
    let prompt = "⚪ Select the type of camera to configure";
    let items = vec![
        CameraSourceTypeEnum::MJPGStreamer,
        CameraSourceTypeEnum::Gstreamer,
    ];
    let idx = Select::new()
        .with_prompt(prompt.to_string())
        .items(&items)
        .default(0)
        .interact()?;
    Ok(items[idx])
}

pub fn prompt_camera_source(camera_source_type: &str) -> Result<String> {
    match camera_source_type {
        
    }
}

pub fn prompt_camera_name() -> Result<String> {
    let name_prompt = "⚪ Enter a name for printer profile";
    Input::new()
        .with_prompt(name_prompt)
        .default("Prusa i3 MK3S".to_string())
        .interact_text()

pub fn prompt_camera_add(device: i32) {
    let name = prompt_camera_name()?;
    let camera_type = prompt_camera_type()?;
    let camera_source_type = prompt_camera_source_type()?;
    let camera_source = prompt_camera_source(&camera_source_type)?;
}
