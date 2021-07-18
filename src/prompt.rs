
use log::{ info };
use sysinfo::{ SystemExt };
use dialoguer::{ Input, MultiSelect };
use print_nanny_client::models::{ 
    PrinterProfileRequest
};

pub fn prompt_email(default_email: &Option<String>) -> String {
    let prompt = "⚪ Please your email address";
    match default_email {
        Some(v) => {
            return Input::new()
            .with_prompt(prompt)
            .default(v.to_string())
            .interact_text()
            .unwrap();
        },
        None => {
            return Input::new()
                .with_prompt(prompt)
                .interact_text()
                .unwrap();
        }
    };
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

pub fn prompt_camera_add(device: i32) {
    let name_prompt = "⚪ Enter a name for printer profile";
    let name = Input::new()
        .with_prompt(name_prompt)
        .default("Prusa i3 MK3S".to_string())
        .interact_text()
        .unwrap();

        let camera_prompt = "⚪ Select streamer software";
        let items = vec!["mjpg-streamer"];
        let chosen : Vec<usize> = MultiSelect::new()
            .with_prompt(camera_prompt.to_string())
            .items(&items)
            .interact()
            .unwrap();

}
