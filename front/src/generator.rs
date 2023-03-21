
use std::{
string::String,
io::{prelude::*, BufWriter},
fs::File,
path::Path
};


extern crate barcoders;
use barcoders::{
    sym::{
        ean13::EAN13,
        ean8::EAN8
    },
    generators::image::*
};





pub struct Parser {
    barcode: String,
    qty: String,
}

impl Parser {
    pub fn new(barcode:String, qty:String) -> Parser {
        Parser {
            barcode,
            qty
        }
    }

    pub fn parse_inputs(&self) -> Result<(String,String, u8), String> {    
        let barcode = self._test_barcode_input();  
        let qty = self._test_qty_input();     
        match barcode {
            Ok(b) => {
                match qty {
                    Ok(q) => Ok((b.0, b.1, q)),
                    Err(q) => Err(q)
                }
            },
            Err(b) => {
                match qty {
                    Ok(_) => Err(b),
                    Err(q) => Err(b + " et " + &q.as_str())
                }
            }
        } 

    }

    fn _test_qty_input(&self) -> Result<u8, String> {
        let qty: Result<u8, std::num::ParseIntError>= self.qty.parse::<u8>();
        match qty {
            Ok(n) => {
                if n <= 32 {
                    Ok(n)
                } else {
                    Err(String::from("Vous êtes limité à un maximum de 32 impréssions par cycles"))
                }
            },
            Err(e) => {
                if self.qty.chars().all(char::is_numeric) == false {
                    Err(String::from("Le champ quantité doit contenir aucun caractères non numériques"))
                } else {
                    Err(String::from("Vous êtes limité à un maximum de 32 impréssions par cycles"))
                }
            } 
        }
    }

    fn _test_barcode_input(&self) -> Result<(String, String), String> {
        if self.barcode.chars().all(char::is_numeric) == false {
            Err(String::from("Un code barre doit être entièrement composé de chiffres"))
        } else {
            match self.barcode.len() {
                8  => Ok((String::from("ean8"), self.barcode.clone())),
                13 => Ok((String::from("ean13"), self.barcode.clone())),
                _  => Err(String::from("Le code barre doit contenir 8 ou 13 caractères"))
            }
        }
    }
}





pub struct Brcdgenerator {
    brcd_type: String,
    barcode: String,
    qty: u8,
}

impl Brcdgenerator {
    pub fn new(brcd_type: String, barcode: String, qty: u8) -> Brcdgenerator {
        Brcdgenerator {
            brcd_type,
            barcode,
            qty
        }
    }

    pub fn generate(&self, height: u32) -> Vec<u8> {
        let encoded = match self.brcd_type.as_str() {
            "ean13" => EAN13::new(self.barcode.clone()).unwrap().encode(),
            "ean8"  => EAN8::new(self.barcode.clone()).unwrap().encode(),
            &_      => vec![0]
        };
        let sym = Image::GIF{height: 90,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color::new([0, 0, 0, 255]),
            background: Color::new([255, 255, 255, 255])};

        sym.generate(&encoded[..]).unwrap()
    }

    fn _symbology(&self) {

    }

    fn _to_file(&self, buf: Vec<u8>) {
        let file = File::create(&Path::new("./tmp/brcd.png")).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&buf[..]).unwrap();
    }
}