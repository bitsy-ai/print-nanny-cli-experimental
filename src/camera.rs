use anyhow::{ Context, Result, anyhow };

use print_nanny_client::models::{ 
    CameraController,
    CameraControllerRequest
};
use crate::config::{ PrintNannySystemConfig };
use crate::prompt::{ prompt_camera_add };


pub async fn camera_add(config: &mut PrintNannySystemConfig) -> Result<()> {
    let device  = config.device_identity.as_ref();

    if device.is_none() {
        Err(anyhow!("🔴 Device not registered. Please run `printnanny auth` to get started"))
    } else {
        Ok(())
        // Ok(prompt_camera_add(device.unwrap().id))
    }
}