
use sysinfo::{ProcessExt, SystemExt};
use dialoguer::Input;


pub fn prompt_token_input(email: &str) -> String {
    let prompt = format!("⚪ Please enter the 6-digit code emailed to {}", email.to_string());
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
    let hostname = system.hostname();
    let prompt = format!("⚪ Please a name for this device:");
    Input::new()
        .with_prompt(prompt)
        .default(hostname)
        .interact_text()
        .unwrap()
}