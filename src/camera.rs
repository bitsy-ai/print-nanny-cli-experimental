use anyhow::{ Context, Result };

use print_nanny_client::models::{ 
    PrinterProfile,
    PrinterProfileRequest
};
use crate::config::{ PrintNannySystemConfig };
use crate::prompt::{ prompt_camera_add };


pub async fn camera_add(config: &mut PrintNannySystemConfig) -> Result<()> {
    let device_id = config.device_identity.as_ref().unwrap();
    // prompt_printer_add(device_id);
    Ok(())
}