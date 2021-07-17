
use sysinfo::{ProcessExt, SystemExt};
use anyhow::{ Result };
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

pub async fn device_identity_update_or_create(config: &PrintNannySystemConfig, name: &str) -> Result<()> {
    let api_config = print_nanny_client::apis::configuration::Configuration{
        base_path:config.api_url.to_string(),
        bearer_access_token:Some(config.api_token.clone()),
        ..Default::default() 
    };
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let os_version = system.os_version();
    let os = system.long_os_version();
    let kernel_version = system.kernel_version();
    let ram = system.total_memory();
    let cores = system.physical_core_count().unwrap();
    let cpuinfo = CpuInfo::new();

    // let req = DeviceRequest{
    //     name,
    //     os_version,
    //     os,
    //     kernel_version,
    //     ram,
    //     cores
    // };
    Ok(())
}