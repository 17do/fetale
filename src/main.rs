#![allow(warnings)]
mod macros;
mod parser;
use clap::{Parser, Subcommand};
use std::{collections::HashMap, env, fs, io};

#[derive(Parser)]
#[command(name = "fet")]
#[command(about = "The fet programming language interpreter")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();
    match args {
        _ => {
            let mut str_ = String::new();
            load_file(&args.filename, &mut str_);
            parser::parse(str_.as_str());
        }
    }
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
