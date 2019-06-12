extern crate toml;
extern crate toml_edit;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::prelude::*;

use toml_edit::{Document, value};

fn main() {
    let manifest_str = match File::open("./Cargo.toml") {
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            do_cat(&mut buf_file)
        },
        Err(e) => panic!("{}", e),
    };
    let mut doc = manifest_str.parse::<Document>().expect("invalid doc");
    doc["package"]["version"] = value("0.1.0");
    let mut file = std::fs::File::create("DummyVersion.toml").unwrap();
    file.write_all(doc.to_string().as_bytes()).expect("Could not write to file!");
}

fn do_cat(stream: &mut BufRead) -> String {
    let mut buffer = String::new();
    let mut result = String::new();
    loop {
        match stream.read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                result = format!("{}{}", result, buffer);
                buffer.clear();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    result
}