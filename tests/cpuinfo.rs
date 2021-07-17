use std::fs;
use anyhow::{ Result };

use printnanny::cpuinfo::{ CpuInfo };


#[test]
fn test_rpi_cpuinfo() -> Result<()> {
    let data = fs::read_to_string("tests/fixtures/rpi_cpuinfo.txt")?;
    let cpuinfo = CpuInfo::from_string(&data);

    println!("Parsed cpuinfo {:#?}", cpuinfo);
    assert!(cpuinfo.processors.len() == 4);

    // let expected_model = "Raspberry Pi 4 Model B Rev 1.1";
    match cpuinfo.extra {
        Some(x) => {
            assert!(x["Hardware"] == "BCM2711");
            assert!(x["Model"] == "Raspberry Pi 4 Model B Rev 1.1");
            assert!(x["Revision"] == "c03111");
            assert!(x["Serial"] == "100000003fa9a39b");
        },
        None => {}
    };
    Ok(())
}

#[test]
fn test_amd64() -> Result<()> {
    let data = fs::read_to_string("tests/fixtures/amd64_cpuinfo.txt")?;
    let cpuinfo = CpuInfo::from_string(&data);

    println!("Parsed cpuinfo {:#?}", cpuinfo);
    assert!(cpuinfo.processors.len() == 64);
    assert!(cpuinfo.extra == None );
    Ok(())
}