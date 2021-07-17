use sysinfo::{SystemExt};
use std::convert::TryFrom;
use anyhow::{ Context, Result };
use print_nanny_client::models::{ 
    DeviceIdentity,
    DeviceRequest
};
use print_nanny_client::apis::devices_api::{ devices_update_or_create };
use crate::config::{ PrintNannySystemConfig };
use crate::cpuinfo::{ CpuInfo };

// TODO
// Component struct implements sysfs-interface
// https://docs.rs/sysinfo/0.19.2/sysinfo/struct.Component.html
// https://www.kernel.org/doc/Documentation/hwmon/sysfs-interface

pub async fn device_identity_update_or_create(config: &PrintNannySystemConfig, name: &str) -> Result<DeviceIdentity> {
    let api_config = print_nanny_client::apis::configuration::Configuration{
        base_path:config.api_url.to_string(),
        bearer_access_token:Some(config.api_token.clone()),
        ..Default::default() 
    };
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let os_version = system.os_version().unwrap();
    let os = system.long_os_version().unwrap();
    let kernel_version = system.kernel_version().unwrap();
    let ram = i64::try_from(system.total_memory())?;
    
    // /proc/cpuinfo
    let cpuinfo = CpuInfo::new()?;
    let cores = cpuinfo.cores().unwrap();
    let cpu_flags = cpuinfo.cpu_flags()?.to_string();
    let hardware = cpuinfo.rpi_hardware();
    let model = cpuinfo.rpi_model();
    let serial = cpuinfo.rpi_serial();
    let revision = cpuinfo.rpi_revision();
    
    let req = DeviceRequest{
        name:name.to_string(),
        os_version,
        os,
        kernel_version,
        ram,
        cores,
        cpu_flags,
        hardware,
        serial,
        model,
        revision
    };
    let res = devices_update_or_create(&api_config, req).await
        .context("🔴 devices_update_or_create request failed")?;
    Ok(res)
}