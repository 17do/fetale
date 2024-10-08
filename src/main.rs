#![allow(warnings)]
mod macros;
mod parser;
use clap::{Parser, Subcommand};
use std::{collections::HashMap, env, fs, io, path::PathBuf, process};

#[derive(Parser)]
#[command(name = "fet")]
#[command(about = "The fetale programming language interpreter")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    #[arg(value_parser = clap::value_parser!(PathBuf))]
    target_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    match args {
        _ => {
            let loaded_text = load_file(&args.target_path);
            parser::parse(loaded_text.as_str());
        }
    }
}

fn load_file(file_path: &PathBuf) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {}", e);
        process::exit(1)
    })
}
