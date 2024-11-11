use thiserror::Error as TError;
use std::error::Error;
use std::process::Command;
pub mod memory;
pub mod environment;
pub mod host;
pub mod cpu;
pub mod gpu;

#[derive(Debug, TError, Clone)]
pub enum HWError {
    #[error("Not Supported")]
    NotSupported,
    #[error("OS is not Linux")]
    NotLinux,
}

pub fn is_linux() -> Result<bool, Box<dyn Error>>{
    let mut uname = String::new();
    match Command::new("uname")
        .arg("-s")
        .output() {
            Ok(x) => x,
            Err(_) => return Err(Box::new(HWError::NotLinux)),
        }
        .stdout
        .iter()
        .for_each(|b| uname.push(*b as char));
    
     Ok(uname.replace("\n", "").trim() == "Linux")
}
