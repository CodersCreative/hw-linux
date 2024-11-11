use std::fs;
use std::process::Command;
use super::{is_linux, HWError};

use std::error::Error;

#[derive(Default, Clone, Debug)]
pub struct Gpu{
    name : Option<String>,
    vendor : Option<String>,
    driver : Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct Gpus(Vec<Gpu>);


impl Gpus{
    pub fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut gpus = Self::default();
        
        let command = Command::new("lspci")
            .arg("-nnk")
            .output()?;
        
        let mut since = 0;
        
        std::str::from_utf8(&command.stdout)?.trim()
            .split('\n')
            .for_each(|i| {
                if i.contains("Display") || i.contains("3D") || i.contains("VGA"){
                    let mut gpu = Gpu::default();
                    let inf = i.split(':').collect::<Vec<&str>>();
                    if inf.len() > 1 {
                        gpu.name = Some(inf[1].trim().to_string());
                        gpu.vendor = Some(inf[1].split_whitespace().collect::<Vec<&str>>()[0].trim().to_string());
                        gpus.0.push(gpu);
                    }
                    since = 0;
                }else{
                    if since < 3{
                        if i.contains("driver"){
                            let inf = i.split(':').collect::<Vec<&str>>();
                            if inf.len() > 1{
                                let mut gpu = gpus.0.last().unwrap().clone();
                                gpu.driver = Some(inf[1].trim().to_string());
                                gpus.0.pop();
                                gpus.0.push(gpu);
                            }
                        }
                    }
                    since += 1;
                }
            });
        Ok(gpus)
    }
}

