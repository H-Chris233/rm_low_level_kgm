#![allow(dead_code)]
#![allow(unused)]

use walkdir::WalkDir;
use std::path::Path;
use clap::Parser;
use rm_low_level_kgm::Cli;
use std::fs;
use colored::Colorize;
use std::fmt::Debug;

fn main() {
    let cli = Cli::parse();    
    let vec_path = check_path(cli.vec_dir);
    for path in vec_path {
        
        for entry in WalkDir::new(&path) // 替换为目标目录
            .into_iter()
            .filter_map(Result::ok)
            .filter(|d| d.path().is_file()) {
    
    
    
        }
        
    }
    
    
    
    
    
}

fn check_path(vec_dir: Vec<String>) -> Vec<String> {
    let mut vec_path: Vec<String> = Vec::new();
    for dir in vec_dir {
        let metadata = match fs::metadata(Path::new(&dir)) {
            Ok(v) => v,
            Err(e) => error(e),
        };
        if metadata.is_dir() {
            vec_path.push(dir);
        }else {error("No such file or dictionary.");}
    }
    
    vec_path
}

fn error<E: Debug>(err: E) -> ! {
    println!("{}: {:?}", "Error".red().bold(), err);
    std::process::exit(1);
}







