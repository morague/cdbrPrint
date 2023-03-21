use std::{
    io::{prelude::*, BufWriter},
    fs::File,
    path::{Path}
    };
use serde::{Serialize, Deserialize};


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

    pub fn _to_img(&self) -> &Path {
        let path = Path::new("./brcd.png");
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&self.buffer[..]).unwrap();

        path
    }
}

