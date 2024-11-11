use std::fs;
use std::process::Command;
use super::{is_linux, HWError};

use std::error::Error;
#[derive(Default, Clone, Debug)]
pub struct MemoryInfo{
    total : Option<f64>,
    used : Option<f64>,
    free : Option<f64>,
}

#[derive(Default, Clone, Debug)]
pub struct SwapInfo{
    total : Option<f64>,
    used : Option<f64>,
    free : Option<f64>,
}

impl SwapInfo{
    pub fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut total = 0_f64;
        let mut free = 0_f64;
        fs::read_to_string("/proc/meminfo")?
            .split('\n')
            .for_each(|i| {
                let inf = i.split(':').collect::<Vec<&str>>();
                if inf.len() > 1 {
                    let key = inf[0].trim();
                    let val = inf[1]
                        .replace("kB", "")
                        .replace("\n", "")
                        .trim()
                        .parse::<f64>()
                        .unwrap();

                    match key {
                        "SwapTotal" => {
                            total = val;
                        }
                        "SwapFree" => {
                            free = val;
                        }
                        &_ => (),
                    }
                }
            });

        let mut swap = Self::default();
        swap.used = Some((total - free) / 1024_f64);
        swap.total = Some(total / 1024_f64);
        swap.free = Some(free / 1024_f64);
        Ok(swap)
    }
}

impl MemoryInfo{
    pub fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut total = 0_f64;
        let mut free = 0_f64;
        fs::read_to_string("/proc/meminfo")?
            .split('\n')
            .for_each(|i| {
                let inf = i.split(':').collect::<Vec<&str>>();
                if inf.len() > 1 {
                    let key = inf[0].trim();
                    let val = inf[1]
                        .replace("kB", "")
                        .replace("\n", "")
                        .trim()
                        .parse::<f64>()
                        .unwrap();

                    match key {
                        "MemTotal" => {
                            total = val;
                        }
                        "MemAvailable" => {
                            free = val;
                        }
                        &_ => (),
                    }
                }
            });

        let mut mem = Self::default();
        mem.used = Some((total - free) / 1024_f64);
        mem.total = Some(total / 1024_f64);
        mem.free = Some(free / 1024_f64);
        Ok(mem)
    }
}
