use std::process::Command;
use crate::{InfoTrait, is_linux};
use std::error::Error;

#[derive(Default, Clone, Debug)]
pub struct HostInfo{
    pub distro : Option<String>,
    pub os : Option<String>,
    pub architecture : Option<String>,
    pub vendor : Option<String>,
    pub model : Option<String>,
    pub desktop_env : Option<String>,
    pub session : Option<String>,
    pub win_manager : Option<String>,
}

impl HostInfo{
    pub fn get_distro() -> Result<String, Box<dyn Error>>{

        let command = Command::new("hostname")
            .output()?;

        Ok(std::str::from_utf8(&command.stdout)?.trim().to_string())
    }
}

impl InfoTrait for HostInfo{
    fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut host = Self::default();
        
        let command = Command::new("hostnamectl")
            .output()?;

        std::str::from_utf8(&command.stdout)?.trim()
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
                        "Operating System" => {
                            host.os = Some(val);
                        },
                        "Architecture" => {
                            host.architecture = Some(val);
                        },
                        "Hardware Vendor" => {
                            host.vendor = Some(val);
                        },
                        "Hardware Model" => {
                            host.model = Some(val)
                        }
                        &_ => (),
                    }
                }
            });

        host.distro = Some(Self::get_distro()?);
        host.desktop_env = Some(std::env::var("DESKTOP_SESSION")?.trim().to_string());
        host.win_manager = Some(std::env::var("USER")?.trim().to_string());
        host.session = Some(std::env::var("XDG_SESSION_TYPE")?.trim().to_string());
        Ok(host)
    }
}

