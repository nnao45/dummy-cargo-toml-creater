extern crate toml;
extern crate toml_edit;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::prelude::*;

use toml_edit::{Document, value};

fn main(){
    ctoml_creater();
    clock_creater();
}

fn ctoml_creater() {
    let manifest_str = match File::open("./Cargo.toml") {
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            do_cat(&mut buf_file)
        },
        Err(e) => panic!("{}", e),
    };
    let mut doc = manifest_str.parse::<Document>().expect("invalid doc");
    doc["package"]["version"] = value("0.1.0");
    let mut file = std::fs::File::create("./DummyVersion.toml").unwrap();
    file.write_all(doc.to_string().as_bytes()).expect("Could not write to file!");
}

fn clock_creater() {
    let manifest_str = match File::open("./Cargo.lock") {
        Ok(file) => {
            let mut buf_file = BufReader::new(file);
            do_cat(&mut buf_file)
        },
        Err(e) => panic!("{}", e),
    };
    let mut doc = manifest_str.parse::<Document>().expect("invalid doc");
    let mut counter = 0;
    let mut idx = 0;
    let tables = doc["package"].as_array_of_tables_mut().unwrap();
    for t in tables.iter() {
        for v in t.iter() {
            if v.0 == "name" {
                if let Some(s) = v.1.as_str() {
                    if s ==  env!("CARGO_PKG_NAME") {
                        idx = counter;
                        break
                    }
                }
            }
        }
        counter += 1;
    }
    doc["package"][idx]["version"] = value("0.1.0");
    let mut file = std::fs::File::create("./DummyVersion.lock").unwrap();
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