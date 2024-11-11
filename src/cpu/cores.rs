use std::error::Error;
use std::fs;
use crate::{is_linux, InfoTrait};

#[derive(Default, Clone, Debug)]
pub struct Core{
    pub name : Option<String>,
    pub usage : Option<u64>
}

#[derive(Default, Clone, Debug)]
pub struct Cores(pub Vec<Core>);

impl InfoTrait for Cores {
    fn get() -> Result<Self, Box<dyn Error>>{
        let _ = is_linux()?;
        let mut cores = Cores::default();
        fs::read_to_string("/proc/stat")?
            .split('\n')
            .for_each(|i| {
                let fields : Vec<&str> = i.split_whitespace().collect();
                if fields[0].contains("cpu"){
                    let user_time = fields[1].trim().parse::<u64>().unwrap();
                    let nice_time = fields[2].trim().parse::<u64>().unwrap();
                    let system_time = fields[3].trim().parse::<u64>().unwrap();
                    let idle_time = fields[4].trim().parse::<u64>().unwrap();

                    let total_time = user_time + nice_time + system_time + idle_time;
                    cores.0.push(Core{ 
                        name : Some(fields[0].to_string()),
                        usage : Some(100 - (idle_time * 100 / total_time)),
                    });
                }
            });

        Ok(cores)
    }
}
