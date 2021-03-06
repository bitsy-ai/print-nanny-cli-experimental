use anyhow::{ Context, Result, anyhow };

use print_nanny_client::models::{ 
    CameraController,
    CameraControllerRequest
};
use crate::config::{ PrintNannySystemConfig };
use crate::prompt::{ prompt_camera_add };
use crate::auth::{ AuthError };


pub async fn camera_add(config: &mut PrintNannySystemConfig) -> Result<()> {
    let device  = config.device_identity.as_ref();

    if device.is_none() {
        Err(AuthError::AuthRequired.into())
    } else {
        Ok(prompt_camera_add(device.unwrap().id))
    }
}