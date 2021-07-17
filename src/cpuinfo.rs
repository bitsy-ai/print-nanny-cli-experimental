use std::fs;
use std::collections::{ HashMap };
use anyhow::{ Result };

#[derive(Debug, Clone)]
pub struct CpuInfo {
   pub processors: Vec<HashMap<String, String>>, // Vec of parsed processor blocks
   pub extra: Option<HashMap<String, String>> // common fields
}

impl CpuInfo {

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
        ( key.trim().to_string(), value[1..].trim().to_string() )
    }

    fn parse_blocks(data: &str) -> Vec<HashMap<String, String>> {
        data.split("\n\n")
            .map(|block| 
                block.trim()
                    .split("\n")
                    .map(CpuInfo::parse_line).collect()).collect()
    }
}