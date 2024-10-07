#![allow(warnings)]
mod macros;
mod parser;
use std::{collections::HashMap, fs, io};

fn main() {
    let mut str_ = String::new();
    load_file("/home/inado/develop/tval/module.fet", &mut str_);
    parser::parse(str_.as_str());
}

#[test]
fn test() {
    let mut str_ = String::new();
    load_file("/home/inado/develop/tval/module.fet", &mut str_);
    println!("{}", str_);
    parser::parse(str_.as_str());
}

fn load_file(file_path: &str, file_str: &mut String) {
    let s = match fs::read_to_string(file_path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            std::process::exit(1);
        }
    };
    *file_str = s;
}
