use std::{process::Command, 
    str::from_utf8,
    string::String,
    io::{prelude::*, BufWriter},
    fs::File,
    path::{Path},
    env};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

type Dimension = (u32, u32);

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Jobstate {
    Success,
    Fail
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    buffer: Vec<u8>,
    qty: u8,
    path: Option<String>,
    state: Option<Jobstate>
}

impl Job {
    pub fn new(buffer: Vec<u8>, qty: u8) -> Job {
        Job {
            buffer: buffer,
            qty: qty,
            path: None,
            state: None
        }
    }    

    pub fn _to_img(&mut self) {
        let tmp_file_path = env::temp_dir().join(Uuid::new_v4().to_string());
        let mut tmp_file = File::create(&tmp_file_path).unwrap();
        let save = tmp_file.write(&self.buffer);
        self.path = Some(String::from(tmp_file_path.as_os_str().to_str().unwrap()));
    }
}



pub struct Printer {
    name: String,
    dim: Option<Dimension>,
    active: bool,
    saved: bool
}

impl Printer {
    pub fn new(name: String) -> Printer {
        Printer {
            name: name,
            dim: None,
            active: false,
            saved: false
        }
    }

    pub fn config_dimension(&mut self, w:u32, h:u32) {
        self.dim = Some((w, h))
    }

    pub fn set_as_active(&mut self) {
        self.active = true
    }

    pub fn set_as_inactive(&mut self) {
        self.active = false
    }

    pub fn verify_printer_config(& self) -> Result<(u32, u32), String> {
        match self.dim {
            None => Err(String::from("Vous devez configurer les dimension de l'imprimante avant de l'utiliser")),
            Some(t) => Ok(t)
        }
    }

    #[cfg(all(unix))]
    pub fn print(&self, job:Job) -> Result<bool, String> {
        ///lp file -d QL-720NW -o orientation-requested=90 -o media=Custom.62x29mm -n 1
        
        if self.dim.is_some() {
            let w:u32 = self.dim.unwrap().0;
            let h:u32 = self.dim.unwrap().1;
            let cmd = Command::new("lp")
                                // .arg(format!("-d {}", self.name))
                                .arg("-o orientation-requested=90")
                                .arg(format!("-o media=Custom.{}x{}mm", w, h))
                                .arg(format!("-n {}", job.qty))
                                .arg(job.path.unwrap())
                                .output()
                                .unwrap();
            println!("{:?}", cmd);
            match cmd.status.success() {
                true => Ok(true),
                false => Err(String::from(from_utf8(&cmd.stderr).unwrap()))
            }
        } else {
            Err(String::from("Vous devez configuerer les dimension de l'imprimante"))
        }
    }

    // #[cfg(all(windows))]
    // pub fn print(&self, job:Job) -> Result<bool, String> {
    //     ///lp file -d QL-720NW -o orientation-requested=90 -o media=Custom.62x29mm -n 1
    //     let cmd = Command::new("lp")
    //                         .arg(format!("-d {}", self.name))
    //                         .arg("-o orientation-requested=90")
    //                         .arg(format!("-o media=Custom.{}x{}mm", job.dim.0, job.dim.1))
    //                         .arg(format!("-n {}", job.qty))
    //                         .arg(job.buffer)
    //                         .output()
    //                         .unwrap();

    //     match cmd.status.success() {
    //         true => Ok(true),
    //         false => Err(String::from(from_utf8(&cmd.stderr).unwrap()))
    //     }
    // }
}