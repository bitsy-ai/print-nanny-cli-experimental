use std::fs;
use std::collections::{ HashMap };
use anyhow::{ Result, anyhow};
#[derive(Debug, Clone)]
pub struct CpuInfo {
   pub processors: Vec<HashMap<String, String>>, // Vec of parsed processor blocks
   pub extra: Option<HashMap<String, String>> // common fields
}

impl CpuInfo {
    pub fn cpu_flags(&self) -> Result<&str> {
        let cpu0 = &self.processors[0];
        if cpu0.contains_key("features"){
            Ok(&cpu0["features"])
        } else if cpu0.contains_key("flags"){
            Ok(&cpu0["flags"])
        } else {
            Err(anyhow!("Failed to parse cpu flags from cpu0 block {:?}", cpu0))
        }
    }

    pub fn from_string(data: &str) -> CpuInfo {
        let vec_map = CpuInfo::parse_blocks(&data);
        let processors = vec_map
            .iter()
            .filter(|x| x.contains_key("processor"))
            .cloned()
            .collect();
        let extra = vec_map.iter().filter(|x| !x.contains_key("processor")).next().cloned();
        CpuInfo {
            processors, extra
        }
    }
    pub fn new() -> Result<CpuInfo> {
        let filename = "/proc/cpuinfo";
        let data = fs::read_to_string(filename)?;
        Ok(CpuInfo::from_string(&data))
    }

    fn parse_line(line: &str) -> (String, String) {
        let errmsg = format!("Failed to parse line {}", line.to_string());
        let splitpoint = line.find(":").expect(&errmsg);
        let (key, value) = line.split_at(splitpoint);
        ( key.trim().to_lowercase(), value[1..].trim().to_string() )
    }

    fn parse_blocks(data: &str) -> Vec<HashMap<String, String>> {
        data.split("\n\n")
            .map(|block| 
                block.trim()
                    .split("\n")
                    .map(CpuInfo::parse_line).collect()).collect()
    }
}