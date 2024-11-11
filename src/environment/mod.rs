use std::fs;
use std::process::Command;
use crate::{is_linux, HWError};
pub mod packages;
use std::error::Error;
#[derive(Default, Clone, Debug)]
pub struct EnvironmentInfo{
    user : Option<String>,
    shell : Option<String>,
    term : Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct KernelInfo{
    version : Option<String>,
    release : Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct UptimeInfo(Option<f64>);

impl EnvironmentInfo{
    pub fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        Ok(Self{
            user : Some(std::env::var("USER")?.trim().to_string()),
            shell : Some(std::env::var("SHELL")?.trim().to_string()),
            term : Some(std::env::var("TERM")?.trim().to_string()),
        })
    }
}

impl KernelInfo{
    pub fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut kernel = KernelInfo::default();
        let release = Some(match fs::read_to_string("/proc/sys/kernel/osrelease"){
            Ok(x) => x.trim().to_string(),
            Err(_) => {
                let command = Command::new("uname")
                    .arg("-r")
                    .output()?;
                let output = std::str::from_utf8(&command.stdout).unwrap();
                output.trim().to_string()
            }
        });
        kernel.release = release.clone();
        kernel.version = Some(release.unwrap().split(".").collect::<Vec<&str>>()[0].to_string());
        Ok(kernel)
    }
}

impl UptimeInfo{
    pub fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut uptime = Self::default();
        uptime.0 = Some(fs::read_to_string("/proc/uptime")?.parse::<f64>()?);
        Ok(uptime)
    }
}
