use std::fs;
use crate::{is_linux, InfoTrait};
use std::error::Error;

pub mod cores;

#[derive(Default, Clone, Debug)]
pub struct CpuInfo{
    pub name : Option<String>,
    pub vendor : Option<String>,
    pub cores : Option<usize>,
    pub threads : Option<usize>,
    pub cache : Option<f64>,
    pub min_freq : Option<f64>,
    pub cur_freq : Option<f64>,
    pub max_freq : Option<f64>,
    pub temp : Option<f64>,
}

impl InfoTrait for CpuInfo{
    fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut cpu = Self::default();
        fs::read_to_string("/proc/cpuinfo")?
            .split('\n')
            .for_each(|i| {
                let inf = i.split(':').collect::<Vec<&str>>();
                if inf.len() > 1 {
                    let key = inf[0].trim();
                    let val = inf[1]
                        .replace("kB", "")
                        .replace("\n", "")
                        .trim().to_string();

                    match key {
                        "model name" => {
                            cpu.name = Some(val);
                        }
                        "vendor_id" => {
                            cpu.vendor = Some(val);
                        }
                        "cpu cores" => {
                            cpu.cores = match val.parse::<usize>(){
                                Ok(x) => Some(x),
                                Err(_) => None,
                            }
                        }
                        "siblings" => {
                            cpu.threads = match val.parse::<usize>(){
                                Ok(x) => Some(x),
                                Err(_) => None,
                            }
                        }
                        "cache size" => {
                            cpu.cache = match val.parse::<f64>(){
                                Ok(x) => Some(x),
                                Err(_) => None,
                            }
                        }
                        &_ => (),
                    }
                }
            });

        cpu.min_freq = Some(fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_min_freq")?.trim().parse::<f64>()? / 1000_f64);
        cpu.max_freq = Some(fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")?.trim().parse::<f64>()? / 1000_f64);
        cpu.cur_freq = Some(fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_cur_freq")?.trim().parse::<f64>()? / 1000_f64);
        //cpu.temp = Some(fs::read_to_string("/sys/class/thermal/thermal_zone10/temp")?.trim().parse::<f64>()? / 1000_f64);
        Ok(cpu)
    }
}
